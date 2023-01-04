use axum::{http::StatusCode, routing, Json};

use super::entities::{CreateStyle, Style, UpdateStyle};
use super::StylesRepo;
use crate::auth::signing::Claims;
use crate::auth::Permission;
use crate::entity_ref::Ref;
use crate::errors::Result;
use crate::extractors::{FiltersQuery, PoolClient};
use crate::organizations::extractors::PathOrganizationId;
use crate::routes::AppRouter;
use crate::{Filters, NestedStyleSummary, StyleFilters};

pub(crate) fn org_routes() -> AppRouter {
    AppRouter::new().nest(
        "/styles",
        AppRouter::new()
            .route("/", routing::get(list).post(create))
            .route("/nested/summary", routing::get(list_nested_summary))
            .route(
                "/:style_ref",
                routing::get(get).put(upsert).patch(update).delete(delete),
            ),
    )
}

/// List styles
pub(crate) async fn list(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
) -> Result<Json<Vec<Style>>> {
    claims.ensure(organization_id, &[Permission::ListStyles])?;
    let styles = StylesRepo.list(&client, organization_id).await?;

    Ok(Json(styles))
}

/// List nested style summaries
pub(crate) async fn list_nested_summary(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
    FiltersQuery(filters): FiltersQuery<StyleFilters>,
) -> Result<Json<Vec<NestedStyleSummary>>> {
    let metadata = claims.ensure(organization_id, &[Permission::ListStyles])?;
    let filters = filters.resolve(&client, organization_id).await?;
    let styles = StylesRepo
        .list_nested_summary(&client, metadata, filters)
        .await?;
    Ok(Json(styles))
}

/// Get style
pub(crate) async fn get(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<Style>,
    claims: Claims,
) -> Result<Json<Style>> {
    claims.ensure(organization_id, &[Permission::GetStyle])?;
    let style = StylesRepo.get(&client, organization_id, &ref_).await?;
    Ok(Json(style))
}

/// Create new Style
pub(crate) async fn create(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(mut client): PoolClient,
    Json(style): Json<CreateStyle>,
) -> Result<(StatusCode, Json<Style>)> {
    claims.ensure(organization_id, &[Permission::CreateStyle])?;
    Ok((
        StatusCode::CREATED,
        Json(
            StylesRepo
                .insert(&mut client, organization_id, claims.user_id(), style)
                .await?,
        ),
    ))
}

/// Create or update Style
pub(crate) async fn upsert(
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Style>,
    claims: Claims,
    PoolClient(mut client): PoolClient,
    Json(style): Json<CreateStyle>,
) -> Result<(StatusCode, Json<Style>)> {
    claims.ensure(
        organization_id,
        &[Permission::CreateStyle, Permission::UpdateStyle],
    )?;
    let (created, style) = StylesRepo
        .upsert(&mut client, organization_id, claims.user_id(), &ref_, style)
        .await?;
    let status = if created {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok((status, Json(style)))
}

/// Update Style
pub(crate) async fn update(
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Style>,
    claims: Claims,
    PoolClient(mut client): PoolClient,
    Json(style): Json<UpdateStyle>,
) -> Result<(StatusCode, Json<Style>)> {
    claims.ensure(organization_id, &[Permission::UpdateStyle])?;
    Ok((
        StatusCode::OK,
        Json(
            StylesRepo
                .update(&mut client, organization_id, &ref_, style)
                .await?,
        ),
    ))
}

/// Delete Style
pub(crate) async fn delete(
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Style>,
    claims: Claims,
    PoolClient(client): PoolClient,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeleteStyle])?;
    StylesRepo.delete(&client, organization_id, &ref_).await?;
    Ok(StatusCode::NO_CONTENT)
}
