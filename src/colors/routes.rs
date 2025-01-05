use axum::{http::StatusCode, routing, Json};

use super::{Color, ColorsRepo, CreateColor, UpdateColor};
use crate::auth::signing::Claims;
use crate::auth::Permission;
use crate::entity_ref::Ref;
use crate::errors::Result;
use crate::extractors::PoolClient;
use crate::organizations::extractors::PathOrganizationId;
use crate::routes::AppRouter;

pub(crate) fn org_routes() -> AppRouter {
    AppRouter::new().nest(
        "/colors",
        AppRouter::new()
            .route("/", routing::get(list).post(create))
            .route(
                "/{color_ref}",
                routing::get(get).put(upsert).patch(update).delete(delete),
            ),
    )
}

/// List Color items
pub(crate) async fn list(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
) -> Result<Json<Vec<Color>>> {
    claims.ensure(organization_id, &[Permission::ListColors])?;
    let colors = ColorsRepo.list(&client, organization_id).await?;

    Ok(Json(colors))
}

/// Create new Color
pub(crate) async fn create(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(client): PoolClient,
    Json(color): Json<CreateColor>,
) -> Result<Json<Color>> {
    claims.ensure(organization_id, &[Permission::CreateColor])?;
    Ok(Json(
        ColorsRepo
            .insert(&client, organization_id, claims.user_id(), color)
            .await?,
    ))
}

/// Get Color item
pub(crate) async fn get(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<Color>,
    claims: Claims,
) -> Result<Json<Color>> {
    claims.ensure(organization_id, &[Permission::GetColor])?;
    let res = ColorsRepo.find(&client, organization_id, &ref_).await?;
    if let Some(color) = res {
        Ok(Json(color))
    } else {
        Err(ref_.not_found_error())
    }
}

/// Update Color
pub(crate) async fn update(
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Color>,
    claims: Claims,
    PoolClient(client): PoolClient,
    Json(color): Json<UpdateColor>,
) -> Result<Json<Color>> {
    claims.ensure(organization_id, &[Permission::UpdateColor])?;
    Ok(Json(
        ColorsRepo
            .update(&client, organization_id, &ref_, color)
            .await?,
    ))
}

/// Create or update Color
pub(crate) async fn upsert(
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Color>,
    claims: Claims,
    PoolClient(client): PoolClient,
    Json(color): Json<CreateColor>,
) -> Result<(StatusCode, Json<Color>)> {
    claims.ensure(
        organization_id,
        &[Permission::CreateColor, Permission::UpdateColor],
    )?;
    let (created, color) = ColorsRepo
        .upsert(&client, organization_id, claims.user_id(), &ref_, color)
        .await?;
    let status = if created {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok((status, Json(color)))
}

/// Delete Color
pub(crate) async fn delete(
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Color>,
    claims: Claims,
    PoolClient(client): PoolClient,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeleteColor])?;
    ColorsRepo.delete(&client, organization_id, &ref_).await?;
    Ok(StatusCode::NO_CONTENT)
}
