use axum::{routing, Json};

use crate::auth::signing::Claims;
use crate::auth::Permission;
use crate::errors::Result;
use crate::extractors::PoolClient;
use crate::organizations::extractors::PathOrganizationId;
use crate::routes::AppRouter;
use crate::FiltersRepo;

use super::ItemFilterChoices;

pub(crate) fn org_routes() -> AppRouter {
    AppRouter::new().nest(
        "/admin",
        AppRouter::new().route("/filters/items", routing::get(list_item_filters)),
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
