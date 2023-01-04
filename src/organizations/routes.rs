use axum::extract::State;
use axum::http::StatusCode;
use axum::routing;
use axum::Json;

use super::entities::{CreateOrganization, Organization, UpdateOrganization};
use super::extractors::PathOrganizationId;
use super::repo::OrgRepo;
use crate::auth::signing::Claims;
use crate::auth::Permission;
use crate::errors::Result;
use crate::extractors::PoolClient;
use crate::routes::AppRouter;
use crate::CloudflareApi;

pub(crate) fn global_routes() -> AppRouter {
    AppRouter::new().nest(
        "/organizations",
        AppRouter::new()
            .route("/", routing::post(create_organization))
            .route(
                "/:organization_id",
                routing::get(get_organization)
                    .patch(update_organization)
                    .delete(delete_organization),
            ),
    )
}

/// Get Organization item
///
/// TODO: Verify that the user has access to this specific organization
///
async fn get_organization(
    PoolClient(client): PoolClient,
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
) -> Result<Json<Organization>> {
    claims.ensure(organization_id, &[Permission::GetOrganization])?;
    let org = OrgRepo.get(&client, organization_id).await?;
    Ok(Json(org))
}

/// Create new Organization
async fn create_organization(
    PoolClient(client): PoolClient,
    State(cloudflare_api): State<CloudflareApi>,
    claims: Claims,
    Json(org): Json<CreateOrganization>,
) -> Result<Json<Organization>> {
    Ok(Json(
        OrgRepo
            .insert(&client, cloudflare_api, org, claims.user_id())
            .await?,
    ))
}

/// Update Organization
///
/// TODO: Verify that the user has access to update this specific organization
///
async fn update_organization(
    PoolClient(client): PoolClient,
    State(cloudflare_api): State<CloudflareApi>,
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    Json(org): Json<UpdateOrganization>,
) -> Result<Json<Organization>> {
    claims.ensure(organization_id, &[Permission::UpdateOrganization])?;
    let organization = OrgRepo
        .update(&client, cloudflare_api, organization_id, org)
        .await?;
    Ok(Json(organization))
}

/// Delete Organization
///
/// TODO: Verify that the user has access to delete this specific organization
///
async fn delete_organization(
    PoolClient(client): PoolClient,
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeleteOrganization])?;
    OrgRepo.delete(&client, organization_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
