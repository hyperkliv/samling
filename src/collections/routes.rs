use axum::extract::State;
use axum::{http::StatusCode, routing, Json};

use super::entities::{Collection, CreateCollection, UpdateCollection};
use super::repo::CollectionsRepo;
use crate::auth::signing::Claims;
use crate::auth::Permission;
use crate::errors::Result;
use crate::extractors::{FiltersQuery, PoolClient};
use crate::organizations::extractors::PathOrganizationId;
use crate::routes::AppRouter;
use crate::{CloudflareApi, CollectionFilters, CollectionSummary};
use crate::{CollectionItem, Filters};
use crate::{CollectionWithItems, NestedStyleSortOrder, Ref, Style};

pub(crate) fn org_routes() -> AppRouter {
    AppRouter::new().nest(
        "/collections",
        AppRouter::new()
            .route(
                "/",
                routing::get(list_collection_summaries).post(create_collection),
            )
            .route("/summaries", routing::get(list_collection_summaries))
            .route(
                "/:collection_ref",
                routing::get(get_collection)
                    .patch(update_collection)
                    .put(upsert_collection)
                    .delete(delete_collection),
            )
            .route(
                "/:collection_ref/with-items",
                routing::get(get_collection_with_items),
            )
            .route(
                "/:collection_ref/summary",
                routing::get(get_collection_summary),
            )
            .route(
                "/:collection_ref/items/:style_ref",
                routing::get(get_collection_item),
            ),
    )
}

/// Get collection and its associated styles, with colors and sizes included
async fn get_collection_with_items(
    PoolClient(client): PoolClient,
    claims: Claims,
    PathOrganizationId(organization_id): PathOrganizationId,
    collection_ref: Ref<Collection>,
    FiltersQuery(filters): FiltersQuery<CollectionFilters>,
    sorter: NestedStyleSortOrder,
) -> Result<Json<CollectionWithItems>> {
    let metadata = claims.ensure(organization_id, &[Permission::ListCollectionItems])?;

    let filters = filters.resolve(&client, organization_id).await?;
    let collection = CollectionsRepo
        .get_with_items(&client, metadata, &collection_ref, filters, sorter)
        .await?;

    Ok(Json(collection))
}

/// Get collection summary
async fn get_collection_summary(
    PoolClient(client): PoolClient,
    claims: Claims,
    PathOrganizationId(organization_id): PathOrganizationId,
    collection_ref: Ref<Collection>,
) -> Result<Json<CollectionSummary>> {
    let metadata = claims.ensure(organization_id, &[Permission::GetCollectionSummary])?;

    let collection = CollectionsRepo
        .get_summary(&client, metadata, &collection_ref)
        .await?;

    Ok(Json(collection))
}

/// List Collection summaries
async fn list_collection_summaries(
    claims: Claims,
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
) -> Result<Json<Vec<CollectionSummary>>> {
    let metadata = claims.ensure(organization_id, &[Permission::ListCollections])?;
    let collections = CollectionsRepo.list_summaries(&client, metadata).await?;

    Ok(Json(collections))
}

/// Get Collection
async fn get_collection(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
    ref_: Ref<Collection>,
) -> Result<Json<Collection>> {
    let metadata = claims.ensure(organization_id, &[Permission::GetCollection])?;
    let res = CollectionsRepo.find(&client, metadata, &ref_).await?;
    if let Some(collection) = res {
        Ok(Json(collection))
    } else {
        Err(ref_.not_found_error())
    }
}

/// Create new Collection
async fn create_collection(
    PoolClient(mut client): PoolClient,
    State(cloudflare_api): State<CloudflareApi>,
    claims: Claims,
    PathOrganizationId(organization_id): PathOrganizationId,
    Json(collection): Json<CreateCollection>,
) -> Result<Json<Collection>> {
    let metadata = claims.ensure(organization_id, &[Permission::CreateCollection])?;
    Ok(Json(
        CollectionsRepo
            .insert(&mut client, cloudflare_api, metadata, collection)
            .await?,
    ))
}

/// Update Collection
async fn update_collection(
    PoolClient(mut client): PoolClient,
    State(cloudflare_api): State<CloudflareApi>,
    claims: Claims,
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Collection>,
    Json(collection): Json<UpdateCollection>,
) -> Result<Json<Collection>> {
    let metadata = claims.ensure(organization_id, &[Permission::UpdateCollection])?;
    Ok(Json(
        CollectionsRepo
            .update(&mut client, cloudflare_api, metadata, &ref_, collection)
            .await?,
    ))
}

/// Upsert Collection
async fn upsert_collection(
    PoolClient(mut client): PoolClient,
    State(cloudflare_api): State<CloudflareApi>,
    claims: Claims,
    PathOrganizationId(organization_id): PathOrganizationId,
    ref_: Ref<Collection>,
    Json(collection): Json<CreateCollection>,
) -> Result<(StatusCode, Json<Collection>)> {
    let metadata = claims.ensure(
        organization_id,
        &[Permission::CreateCollection, Permission::UpdateCollection],
    )?;
    let (created, entity) = CollectionsRepo
        .upsert(&mut client, cloudflare_api, metadata, &ref_, collection)
        .await?;
    if created {
        Ok((StatusCode::CREATED, Json(entity)))
    } else {
        Ok((StatusCode::OK, Json(entity)))
    }
}

/// Delete Collection
async fn delete_collection(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
    ref_: Ref<Collection>,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeleteCollection])?;
    CollectionsRepo
        .delete(&client, organization_id, &ref_)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Get style item belonging to a collection, with colors and sizes included
async fn get_collection_item(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
    collection_ref: Ref<Collection>,
    style_ref: Ref<Style>,
) -> Result<Json<CollectionItem>> {
    let metadata = claims.ensure(organization_id, &[Permission::GetCollectionItem])?;

    let item = CollectionsRepo
        .get_collection_item(&client, metadata, &collection_ref, &style_ref)
        .await?;

    Ok(Json(item))
}
