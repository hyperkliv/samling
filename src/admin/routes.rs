use axum::response::IntoResponse;
use axum::{body::StreamBody, routing, Json};
use futures::TryStreamExt;

use crate::auth::signing::Claims;
use crate::auth::Permission;
use crate::errors::Result;
use crate::extractors::PoolClient;
use crate::organizations::extractors::PathOrganizationId;
use crate::routes::AppRouter;
use crate::FiltersRepo;

use super::{ItemFilterChoices, OrganizationDataRepo};

pub(crate) fn org_routes() -> AppRouter {
    AppRouter::new().nest(
        "/admin",
        AppRouter::new()
            .route("/filters/items", routing::get(list_item_filters))
            .route("/data", routing::get(get_organization_data)),
    )
}

/// List filter choices
pub(crate) async fn list_item_filters(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
) -> Result<Json<ItemFilterChoices>> {
    claims.ensure(organization_id, &[Permission::ListFilterChoices])?;
    Ok(Json(
        FiltersRepo
            .list_item_filters(&client, organization_id)
            .await?,
    ))
}

pub(crate) async fn get_organization_data(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
) -> Result<impl IntoResponse> {
    claims.ensure(organization_id, &[Permission::GetAllOrganizationData])?;
    let mut repo = OrganizationDataRepo::new(client, organization_id);
    let fut = repo.stream_organization_data().await?;
    let stream = fut.map_ok(|val| serde_json::to_string_pretty(&val).unwrap());
    Ok(StreamBody::new(stream))
}
