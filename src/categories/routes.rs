use axum::{http::StatusCode, routing, Json};

use super::{CategoriesRepo, Category, CreateCategory, UpdateCategory};
use crate::auth::signing::Claims;
use crate::auth::Permission;
use crate::entity_ref::Ref;
use crate::errors::Result;
use crate::extractors::PoolClient;
use crate::organizations::extractors::PathOrganizationId;
use crate::routes::AppRouter;

pub(crate) fn org_routes() -> AppRouter {
    AppRouter::new().nest(
        "/categories",
        AppRouter::new()
            .route("/", routing::get(list).post(create))
            .route(
                "/:category_ref",
                routing::get(get).put(upsert).patch(update).delete(delete),
            ),
    )
}

/// List Category items
pub(crate) async fn list(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
) -> Result<Json<Vec<Category>>> {
    claims.ensure(organization_id, &[Permission::ListCategories])?;
    Ok(Json(CategoriesRepo.list(&client, organization_id).await?))
}

/// Create new Category
pub(crate) async fn create(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(client): PoolClient,
    Json(category): Json<CreateCategory>,
) -> Result<Json<Category>> {
    claims.ensure(organization_id, &[Permission::CreateCategory])?;
    Ok(Json(
        CategoriesRepo
            .insert(&client, organization_id, claims.user_id(), category)
            .await?,
    ))
}

/// Get Category item
pub(crate) async fn get(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<Category>,
    claims: Claims,
) -> Result<Json<Category>> {
    claims.ensure(organization_id, &[Permission::GetCategory])?;
    let category = CategoriesRepo.get(&client, organization_id, &ref_).await?;
    Ok(Json(category))
}

/// Update Category
pub(crate) async fn update(
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Category>,
    PoolClient(client): PoolClient,
    claims: Claims,
    Json(category): Json<UpdateCategory>,
) -> Result<Json<Category>> {
    claims.ensure(organization_id, &[Permission::UpdateCategory])?;
    Ok(Json(
        CategoriesRepo
            .update(&client, organization_id, &ref_, category)
            .await?,
    ))
}

/// Create or update Category
pub(crate) async fn upsert(
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Category>,
    claims: Claims,
    PoolClient(client): PoolClient,
    Json(category): Json<CreateCategory>,
) -> Result<(StatusCode, Json<Category>)> {
    claims.ensure(
        organization_id,
        &[Permission::CreateCategory, Permission::UpdateCategory],
    )?;
    let (created, category) = CategoriesRepo
        .upsert(&client, organization_id, claims.user_id(), &ref_, category)
        .await?;
    let status = if created {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok((status, Json(category)))
}

/// Delete Category
pub(crate) async fn delete(
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Category>,
    PoolClient(client): PoolClient,
    claims: Claims,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeleteCategory])?;
    CategoriesRepo
        .delete(&client, organization_id, &ref_)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
