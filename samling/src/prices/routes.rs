use axum::{http::StatusCode, routing, Json};

use super::entities::{CreatePrice, Price, UpdatePrice};
use super::repo::{PriceListsRepo, PricesRepo};
use super::{CreatePriceList, PriceList, UpdatePriceList};
use crate::auth::signing::Claims;
use crate::auth::Permission;
use crate::errors::Result;
use crate::extractors::PoolClient;
use crate::organizations::extractors::PathOrganizationId;
use crate::routes::AppRouter;
use crate::{PriceListSummary, Ref};

pub(crate) fn org_routes() -> AppRouter {
    AppRouter::new()
        .nest(
            "/prices",
            AppRouter::new()
                .route("/", routing::get(list_prices).post(create_price))
                .route(
                    "/{price_ref}",
                    routing::get(get_price)
                        .patch(update_price)
                        .put(upsert_price)
                        .delete(delete_price),
                ),
        )
        .nest(
            "/pricelists",
            AppRouter::new()
                .route("/", routing::get(list_pricelists).post(create_pricelist))
                .route("/summary", routing::get(list_pricelist_summaries))
                .route(
                    "/{price_list_ref}",
                    routing::get(get_pricelist)
                        .patch(update_pricelist)
                        .put(upsert_pricelist)
                        .delete(delete_pricelist),
                ),
        )
}

/// List Prices
async fn list_prices(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(client): PoolClient,
) -> Result<Json<Vec<Price>>> {
    claims.ensure(organization_id, &[Permission::ListPrices])?;
    let prices = PricesRepo.list(&client, organization_id).await?;

    Ok(Json(prices))
}

/// Get Price
async fn get_price(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
    ref_: Ref<Price>,
) -> Result<Json<Price>> {
    claims.ensure(organization_id, &[Permission::GetPrice])?;
    let res = PricesRepo.find(&client, organization_id, &ref_).await?;
    if let Some(price) = res {
        Ok(Json(price))
    } else {
        Err(ref_.not_found_error())
    }
}

/// Create new Price
async fn create_price(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(mut client): PoolClient,
    Json(price): Json<CreatePrice>,
) -> Result<Json<Price>> {
    claims.ensure(organization_id, &[Permission::CreatePrice])?;
    Ok(Json(
        PricesRepo
            .insert(&mut client, organization_id, claims.user_id(), price)
            .await?,
    ))
}

/// Update Price
async fn update_price(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(mut client): PoolClient,
    ref_: Ref<Price>,
    claims: Claims,
    Json(price): Json<UpdatePrice>,
) -> Result<Json<Price>> {
    claims.ensure(organization_id, &[Permission::UpdatePrice])?;
    Ok(Json(
        PricesRepo
            .update(&mut client, organization_id, &ref_, price)
            .await?,
    ))
}

/// Upsert Price
async fn upsert_price(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(mut client): PoolClient,
    ref_: Ref<Price>,
    Json(price): Json<CreatePrice>,
) -> Result<(StatusCode, Json<Price>)> {
    claims.ensure(
        organization_id,
        &[Permission::CreatePrice, Permission::UpdatePrice],
    )?;
    let (created, entity) = PricesRepo
        .upsert(&mut client, organization_id, claims.user_id(), &ref_, price)
        .await?;
    if created {
        Ok((StatusCode::CREATED, Json(entity)))
    } else {
        Ok((StatusCode::OK, Json(entity)))
    }
}

/// Delete Price
async fn delete_price(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<Price>,
    claims: Claims,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeletePrice])?;
    PricesRepo.delete(&client, organization_id, &ref_).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// List PriceLists
// TODO: Implement filters
async fn list_pricelists(
    claims: Claims,
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
) -> Result<Json<Vec<PriceList>>> {
    let metadata = claims.ensure(organization_id, &[Permission::ListPriceLists])?;
    let pricelists = PriceListsRepo.list(&client, metadata).await?;

    Ok(Json(pricelists))
}

/// List PriceList summaries
async fn list_pricelist_summaries(
    claims: Claims,
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
) -> Result<Json<Vec<PriceListSummary>>> {
    let metadata = claims.ensure(organization_id, &[Permission::ListPriceLists])?;
    let pricelists = PriceListsRepo.list_summaries(&client, metadata).await?;

    Ok(Json(pricelists))
}

/// Get PriceList
async fn get_pricelist(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<PriceList>,
    claims: Claims,
) -> Result<Json<PriceList>> {
    claims.ensure(organization_id, &[Permission::GetPriceList])?;
    let res = PriceListsRepo.find(&client, organization_id, &ref_).await?;
    if let Some(pricelist) = res {
        Ok(Json(pricelist))
    } else {
        Err(ref_.not_found_error())
    }
}

/// Create new PriceList
async fn create_pricelist(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(mut client): PoolClient,
    Json(pricelist): Json<CreatePriceList>,
) -> Result<Json<PriceList>> {
    claims.ensure(organization_id, &[Permission::CreatePriceList])?;
    Ok(Json(
        PriceListsRepo
            .insert(&mut client, organization_id, claims.user_id(), pricelist)
            .await?,
    ))
}

/// Update PriceList
async fn update_pricelist(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(mut client): PoolClient,
    ref_: Ref<PriceList>,
    claims: Claims,
    Json(pricelist): Json<UpdatePriceList>,
) -> Result<Json<PriceList>> {
    claims.ensure(organization_id, &[Permission::UpdatePriceList])?;
    Ok(Json(
        PriceListsRepo
            .update(&mut client, organization_id, &ref_, pricelist)
            .await?,
    ))
}

/// Upsert PriceList
async fn upsert_pricelist(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(mut client): PoolClient,
    ref_: Ref<PriceList>,
    Json(pricelist): Json<CreatePriceList>,
) -> Result<(StatusCode, Json<PriceList>)> {
    claims.ensure(
        organization_id,
        &[Permission::CreatePriceList, Permission::UpdatePriceList],
    )?;
    let (created, entity) = PriceListsRepo
        .upsert(
            &mut client,
            organization_id,
            claims.user_id(),
            &ref_,
            pricelist,
        )
        .await?;
    if created {
        Ok((StatusCode::CREATED, Json(entity)))
    } else {
        Ok((StatusCode::OK, Json(entity)))
    }
}

/// Delete PriceList
async fn delete_pricelist(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<PriceList>,
    claims: Claims,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeletePriceList])?;
    PriceListsRepo
        .delete(&client, organization_id, &ref_)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
