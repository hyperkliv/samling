use axum::{extract::State, http::StatusCode, routing, Json, Router};
use futures::future::try_join3;

use crate::{
    extractors::{FiltersQuery, PoolClient},
    organizations::extractors::PathOrganizationId,
    routes::AppRouter,
    sso::{GoogleClaims, GoogleCredentials},
    AuthenticatedUser, CloudflareApi, CreateGroup, Environment, Error, Group, GroupSummary,
    MicrosoftClaims, MicrosoftCredentials, Ref, Result, UpdateGroup, UpdateOwnUser,
};

use super::{
    extractors::PathUserId,
    filters::UserFilters,
    hashing::Hasher,
    rbac::Permission,
    repo::AuthRepo,
    signing::{Claims, JwtSigner},
    sorting::UserSortOrder,
    CreateUser, UpdateUser, User, UserLogin,
};

pub(crate) fn global_routes() -> AppRouter {
    Router::new()
        .route("/users/me", routing::get(get_me).patch(update_me))
        .route("/auth/login", routing::post(login))
        .route("/auth/login/google", routing::post(login_google))
        .route("/auth/login/microsoft", routing::post(login_microsoft))
}

pub(crate) fn org_routes() -> AppRouter {
    Router::new()
        .nest(
            "/users",
            Router::new()
                .route("/", routing::get(list_users).post(create_user))
                .route(
                    "/:user_id",
                    routing::get(get_user)
                        .patch(update_user)
                        .delete(delete_user),
                ),
        )
        .nest(
            "/groups",
            Router::new()
                .route("/", routing::get(list_groups).post(create_group))
                .route("/summary", routing::get(list_group_summaries))
                .route(
                    "/:group_ref",
                    routing::get(get_group)
                        .put(upsert_group)
                        .patch(update_group)
                        .delete(delete_group),
                ),
        )
}

pub(crate) async fn get_me(
    PoolClient(client): PoolClient,
    claims: Claims,
    State(environment): State<Environment>,
    State(jwt_signer): State<JwtSigner>,
) -> Result<Json<AuthenticatedUser>> {
    let user = AuthRepo.get_user(&client, claims.user_id()).await?;
    let token = jwt_signer.encode(claims)?;
    Ok(Json(user.authenticated(environment, token)))
}

pub(crate) async fn update_me(
    PoolClient(client): PoolClient,
    claims: Claims,
    State(hasher): State<Hasher>,
    Json(user): Json<UpdateOwnUser>,
) -> Result<Json<User>> {
    let password_hash = match &user.password {
        Some(pwd) => Some(hasher.argon2_hash(pwd)?),
        None => None,
    };
    let user = AuthRepo
        .update_user(&client, claims.user_id(), user.into(), password_hash)
        .await?;
    Ok(Json(user))
}

pub(crate) async fn login(
    PoolClient(client): PoolClient,
    State(hasher): State<Hasher>,
    State(jwt_signer): State<JwtSigner>,
    State(environment): State<Environment>,
    Json(user_login): Json<UserLogin>,
) -> Result<Json<AuthenticatedUser>> {
    let user = AuthRepo.authenticate(&client, hasher, &user_login).await?;
    tokio::spawn(AuthRepo.update_last_sign_in(client, user.id));
    let claims = jwt_signer.claims(&user)?;
    let token = jwt_signer.encode(claims)?;
    Ok(Json(user.authenticated(environment, token)))
}

pub(crate) async fn login_google(
    PoolClient(mut client): PoolClient,
    State(jwt_signer): State<JwtSigner>,
    State(cloudflare_api): State<CloudflareApi>,
    State(environment): State<Environment>,
    Json(google_credentials): Json<GoogleCredentials>,
) -> Result<(StatusCode, Json<AuthenticatedUser>)> {
    let claims = GoogleClaims::verify(
        // TODO: Don't hardcode audience
        &["564594123007-2i1qkn42nrr5gtiq58i1afl7d92es5e4.apps.googleusercontent.com"],
        &google_credentials,
    )
    .await?;

    let maybe_existing_user = AuthRepo.find_user_by_email(&client, &claims.email).await?;

    let profile_image = match claims.picture.parse::<url::Url>() {
        Ok(url) => {
            if let Some(existing_user) = maybe_existing_user {
                if let Some(existing_image) = existing_user.profile_image {
                    Some(existing_image)
                } else {
                    Some(
                        cloudflare_api
                            .upload_image(claims.email.clone(), url.into())
                            .await?,
                    )
                }
            } else {
                Some(
                    cloudflare_api
                        .upload_image(claims.email.clone(), url.into())
                        .await?,
                )
            }
        }
        Err(_) => maybe_existing_user.and_then(|u| u.profile_image),
    };

    let user = CreateUser {
        email: claims.email,
        password: None,
        name: format!("{} {}", claims.given_name, claims.family_name),
        profile_image,
        roles: None,
        groups: None,
    };
    let (created, user) = AuthRepo.upsert_user(&mut client, user, None).await?;
    let claims = jwt_signer.claims(&user)?;
    let token = jwt_signer.encode(claims)?;
    tokio::spawn(AuthRepo.update_last_sign_in(client, user.id));
    let authenticated_user = user.authenticated(environment, token);
    if created {
        Ok((StatusCode::CREATED, Json(authenticated_user)))
    } else {
        Ok((StatusCode::OK, Json(authenticated_user)))
    }
}

pub(crate) async fn login_microsoft(
    PoolClient(mut client): PoolClient,
    State(jwt_signer): State<JwtSigner>,
    State(cloudflare_api): State<CloudflareApi>,
    State(environment): State<Environment>,
    Json(microsoft_credentials): Json<MicrosoftCredentials>,
) -> Result<(StatusCode, Json<AuthenticatedUser>)> {
    let (maybe_existing_user, claims, maybe_profile_image) = try_join3(
        AuthRepo.find_user_by_email(
            &client,
            &microsoft_credentials.unverified_id_token_claims().email,
        ),
        MicrosoftClaims::verify(
            // TODO: Don't hardcode audience
            &["1123ca08-d517-4363-878b-ee22b7a97145"],
            &microsoft_credentials,
        ),
        super::sso::microsoft::get_profile_image(&microsoft_credentials.access_token),
    )
    .await?;
    let profile_image_url = if let Some(profile_image) = maybe_profile_image {
        if let Some(existing_user) = maybe_existing_user {
            if microsoft_credentials.unverified_id_token_claims().email != claims.email {
                // NOTE: Not an unknown error! This is most likely a spoofing attempt.
                return Err(Error::InvalidToken("Unknown error occurred".into()));
            }
            if let Some(existing_image) = existing_user.profile_image {
                Some(existing_image)
            } else {
                Some(
                    cloudflare_api
                        .upload_image(claims.email.clone(), profile_image.into())
                        .await?,
                )
            }
        } else {
            None
        }
    } else {
        maybe_existing_user.and_then(|u| u.profile_image)
    };
    let user = CreateUser {
        email: claims.email,
        password: None,
        name: claims.name,
        profile_image: profile_image_url,
        roles: None,
        groups: None,
    };
    let (created, user) = AuthRepo.upsert_user(&mut client, user, None).await?;
    let claims = jwt_signer.claims(&user)?;
    let token = jwt_signer.encode(claims)?;
    tokio::spawn(AuthRepo.update_last_sign_in(client, user.id));
    let authenticated_user = user.authenticated(environment, token);
    if created {
        Ok((StatusCode::CREATED, Json(authenticated_user)))
    } else {
        Ok((StatusCode::OK, Json(authenticated_user)))
    }
}

pub(crate) async fn list_users(
    PoolClient(client): PoolClient,
    PathOrganizationId(organization_id): PathOrganizationId,
    FiltersQuery(filters): FiltersQuery<UserFilters>,
    sorter: UserSortOrder,
    claims: Claims,
) -> Result<Json<Vec<User>>> {
    claims.ensure(organization_id, &[Permission::ListUsers])?;
    let users = AuthRepo
        .fetch_org_users(&client, organization_id, filters, sorter)
        .await?;
    Ok(Json(users))
}

pub(crate) async fn create_user(
    PoolClient(mut client): PoolClient,
    claims: Claims,
    PathOrganizationId(organization_id): PathOrganizationId,
    State(hasher): State<Hasher>,
    Json(user): Json<CreateUser>,
) -> Result<Json<User>> {
    claims.ensure(organization_id, &[Permission::CreateUser])?;
    let password_hash = match &user.password {
        Some(pwd) => Some(hasher.argon2_hash(pwd)?),
        None => None,
    };
    let user = AuthRepo
        .insert_org_user(&mut client, organization_id, user, password_hash)
        .await?;
    Ok(Json(user))
}

pub(crate) async fn get_user(
    PoolClient(client): PoolClient,
    claims: Claims,
    PathOrganizationId(organization_id): PathOrganizationId,
    PathUserId(user_id): PathUserId,
) -> Result<Json<User>> {
    claims.ensure(organization_id, &[Permission::GetUser])?;
    Ok(Json(
        AuthRepo
            .get_org_user(&client, organization_id, user_id)
            .await?,
    ))
}

pub(crate) async fn update_user(
    PoolClient(mut client): PoolClient,
    claims: Claims,
    PathOrganizationId(organization_id): PathOrganizationId,
    PathUserId(user_id): PathUserId,
    State(hasher): State<Hasher>,
    Json(user): Json<UpdateUser>,
) -> Result<Json<User>> {
    claims.ensure(organization_id, &[Permission::UpdateUser])?;
    let password_hash = match &user.password {
        Some(pwd) => Some(hasher.argon2_hash(pwd)?),
        None => None,
    };
    let user = AuthRepo
        .update_org_user(&mut client, organization_id, user_id, user, password_hash)
        .await?;
    Ok(Json(user))
}

pub(crate) async fn delete_user(
    PoolClient(client): PoolClient,
    claims: Claims,
    PathOrganizationId(organization_id): PathOrganizationId,
    PathUserId(user_id): PathUserId,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeleteUser])?;
    AuthRepo.delete_user(&client, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// List Group items
pub(crate) async fn list_groups(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
) -> Result<Json<Vec<Group>>> {
    claims.ensure(organization_id, &[Permission::ListGroups])?;
    let groups = AuthRepo.list_groups(&client, organization_id).await?;

    Ok(Json(groups))
}

/// List Group items
pub(crate) async fn list_group_summaries(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
) -> Result<Json<Vec<GroupSummary>>> {
    claims.ensure(organization_id, &[Permission::ListGroups])?;
    let groups = AuthRepo
        .list_group_summaries(&client, organization_id)
        .await?;

    Ok(Json(groups))
}

/// Get Group item
pub(crate) async fn get_group(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<Group>,
    claims: Claims,
) -> Result<Json<Group>> {
    claims.ensure(organization_id, &[Permission::GetGroup])?;
    let group = AuthRepo.get_group(&client, organization_id, &ref_).await?;
    Ok(Json(group))
}

/// Create new Group
async fn create_group(
    PoolClient(mut client): PoolClient,
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    Json(group): Json<CreateGroup>,
) -> Result<Json<Group>> {
    claims.ensure(organization_id, &[Permission::CreateGroup])?;
    Ok(Json(
        AuthRepo
            .insert_group(&mut client, organization_id, group, claims.user_id())
            .await?,
    ))
}

/// Update Group
async fn update_group(
    PoolClient(mut client): PoolClient,
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Group>,
    claims: Claims,
    Json(group): Json<UpdateGroup>,
) -> Result<Json<Group>> {
    claims.ensure(organization_id, &[Permission::UpdateGroup])?;
    let group = AuthRepo
        .update_group(&mut client, organization_id, &ref_, group)
        .await?;
    Ok(Json(group))
}

/// Create or update Group
pub(crate) async fn upsert_group(
    PoolClient(mut client): PoolClient,
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Group>,
    claims: Claims,
    Json(group): Json<CreateGroup>,
) -> Result<(StatusCode, Json<Group>)> {
    claims.ensure(
        organization_id,
        &[Permission::CreateGroup, Permission::UpdateGroup],
    )?;
    let (created, group) = AuthRepo
        .upsert_group(&mut client, organization_id, claims.user_id(), &ref_, group)
        .await?;
    let status = if created {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok((status, Json(group)))
}

/// Delete Group
async fn delete_group(
    PoolClient(client): PoolClient,
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Group>,
    claims: Claims,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeleteGroup])?;
    AuthRepo
        .delete_group(&client, organization_id, &ref_)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
