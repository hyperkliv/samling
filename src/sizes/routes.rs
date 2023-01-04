use axum::{http::StatusCode, routing, Json};

use super::{CreateSize, Size, SizesRepo, UpdateSize};
use crate::auth::signing::Claims;
use crate::auth::Permission;
use crate::entity_ref::Ref;
use crate::errors::Result;
use crate::extractors::PoolClient;
use crate::organizations::extractors::PathOrganizationId;
use crate::routes::AppRouter;

pub(crate) fn org_routes() -> AppRouter {
    AppRouter::new().nest(
        "/sizes",
        AppRouter::new()
            .route("/", routing::get(list).post(create))
            .route(
                "/:size_ref",
                routing::get(get).put(upsert).patch(update).delete(delete),
            ),
    )
}

/// List Size items
pub(crate) async fn list(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
) -> Result<Json<Vec<Size>>> {
    claims.ensure(organization_id, &[Permission::ListSizes])?;
    let sizes = SizesRepo.list(&client, organization_id).await?;

    Ok(Json(sizes))
}

/// Create new Size
pub(crate) async fn create(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(client): PoolClient,
    Json(size): Json<CreateSize>,
) -> Result<Json<Size>> {
    claims.ensure(organization_id, &[Permission::CreateSize])?;
    Ok(Json(
        SizesRepo
            .insert(&client, organization_id, claims.user_id(), size)
            .await?,
    ))
}

/// Get Size item
pub(crate) async fn get(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<Size>,
    claims: Claims,
) -> Result<Json<Size>> {
    claims.ensure(organization_id, &[Permission::GetSize])?;
    let res = SizesRepo.find(&client, organization_id, &ref_).await?;
    if let Some(size) = res {
        Ok(Json(size))
    } else {
        Err(ref_.not_found_error())
    }
}

/// Update Size
pub(crate) async fn update(
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Size>,
    PoolClient(client): PoolClient,
    claims: Claims,
    Json(size): Json<UpdateSize>,
) -> Result<Json<Size>> {
    claims.ensure(organization_id, &[Permission::UpdateSize])?;
    Ok(Json(
        SizesRepo
            .update(&client, organization_id, &ref_, size)
            .await?,
    ))
}

/// Create or update Size
pub(crate) async fn upsert(
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Size>,
    claims: Claims,
    PoolClient(client): PoolClient,
    Json(size): Json<CreateSize>,
) -> Result<(StatusCode, Json<Size>)> {
    claims.ensure(
        organization_id,
        &[Permission::CreateSize, Permission::UpdateSize],
    )?;
    let (created, size) = SizesRepo
        .upsert(&client, organization_id, claims.user_id(), &ref_, size)
        .await?;
    let status = if created {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok((status, Json(size)))
}

/// Delete Size
pub(crate) async fn delete(
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Size>,
    claims: Claims,
    PoolClient(client): PoolClient,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeleteSize])?;
    SizesRepo.delete(&client, organization_id, &ref_).await?;
    Ok(StatusCode::NO_CONTENT)
}
