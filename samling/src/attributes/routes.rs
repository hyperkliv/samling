use axum::{http::StatusCode, routing, Json};

use super::entities::{Attribute, CreateAttribute, UpdateAttribute};
use super::repo::{AttributeTypesRepo, AttributesRepo};
use super::{AttributeType, CreateAttributeType, UpdateAttributeType};
use crate::auth::signing::Claims;
use crate::auth::Permission;
use crate::errors::Result;
use crate::extractors::PoolClient;
use crate::organizations::extractors::PathOrganizationId;
use crate::routes::AppRouter;
use crate::Ref;

pub(crate) fn org_routes() -> AppRouter {
    AppRouter::new()
        .nest(
            "/attributes",
            AppRouter::new()
                .route("/", routing::get(list_attributes).post(create_attribute))
                .route(
                    "/{attribute_ref}",
                    routing::get(get_attribute)
                        .patch(update_attribute)
                        .put(upsert_attribute)
                        .delete(delete_attribute),
                ),
        )
        .nest(
            "/attribute-types",
            AppRouter::new()
                .route(
                    "/",
                    routing::get(list_attribute_types).post(create_attribute_type),
                )
                .route(
                    "/{attribute_type_ref}",
                    routing::get(get_attribute_type)
                        .patch(update_attribute_type)
                        .put(upsert_attribute_type)
                        .delete(delete_attribute_type),
                ),
        )
}

/// List Prices
async fn list_attributes(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
) -> Result<Json<Vec<Attribute>>> {
    claims.ensure(organization_id, &[Permission::ListPrices])?;
    let attributes = AttributesRepo.list(&client, organization_id).await?;

    Ok(Json(attributes))
}

/// Get Price
async fn get_attribute(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<Attribute>,
    claims: Claims,
) -> Result<Json<Attribute>> {
    claims.ensure(organization_id, &[Permission::GetPrice])?;
    let res = AttributesRepo.find(&client, organization_id, &ref_).await?;
    if let Some(attribute) = res {
        Ok(Json(attribute))
    } else {
        Err(ref_.not_found_error())
    }
}

/// Create new Price
async fn create_attribute(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(mut client): PoolClient,
    Json(attribute): Json<CreateAttribute>,
) -> Result<Json<Attribute>> {
    claims.ensure(organization_id, &[Permission::CreatePrice])?;
    Ok(Json(
        AttributesRepo
            .insert(&mut client, organization_id, claims.user_id(), attribute)
            .await?,
    ))
}

/// Update Price
async fn update_attribute(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(mut client): PoolClient,
    ref_: Ref<Attribute>,
    claims: Claims,
    Json(attribute): Json<UpdateAttribute>,
) -> Result<Json<Attribute>> {
    claims.ensure(organization_id, &[Permission::UpdatePrice])?;
    Ok(Json(
        AttributesRepo
            .update(&mut client, organization_id, &ref_, attribute)
            .await?,
    ))
}

/// Upsert Price
async fn upsert_attribute(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(mut client): PoolClient,
    ref_: Ref<Attribute>,
    Json(attribute): Json<CreateAttribute>,
) -> Result<(StatusCode, Json<Attribute>)> {
    claims.ensure(
        organization_id,
        &[Permission::CreatePrice, Permission::UpdatePrice],
    )?;
    let (created, entity) = AttributesRepo
        .upsert(
            &mut client,
            organization_id,
            claims.user_id(),
            &ref_,
            attribute,
        )
        .await?;
    if created {
        Ok((StatusCode::CREATED, Json(entity)))
    } else {
        Ok((StatusCode::OK, Json(entity)))
    }
}

/// Delete Price
async fn delete_attribute(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<Attribute>,
    claims: Claims,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeletePrice])?;
    AttributesRepo
        .delete(&client, organization_id, &ref_)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// List PriceLists
async fn list_attribute_types(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
) -> Result<Json<Vec<AttributeType>>> {
    claims.ensure(organization_id, &[Permission::ListPriceLists])?;
    let attribute_types = AttributeTypesRepo.list(&client, organization_id).await?;

    Ok(Json(attribute_types))
}

/// Get PriceList
async fn get_attribute_type(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<AttributeType>,
    claims: Claims,
) -> Result<Json<AttributeType>> {
    claims.ensure(organization_id, &[Permission::GetPriceList])?;
    let res = AttributeTypesRepo
        .find(&client, organization_id, &ref_)
        .await?;
    if let Some(attribute_type) = res {
        Ok(Json(attribute_type))
    } else {
        Err(ref_.not_found_error())
    }
}

/// Create new PriceList
async fn create_attribute_type(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(mut client): PoolClient,
    Json(attribute_type): Json<CreateAttributeType>,
) -> Result<Json<AttributeType>> {
    claims.ensure(organization_id, &[Permission::CreatePriceList])?;
    Ok(Json(
        AttributeTypesRepo
            .insert(
                &mut client,
                organization_id,
                claims.user_id(),
                attribute_type,
            )
            .await?,
    ))
}

/// Update PriceList
async fn update_attribute_type(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(mut client): PoolClient,
    ref_: Ref<AttributeType>,
    claims: Claims,
    Json(attribute_type): Json<UpdateAttributeType>,
) -> Result<Json<AttributeType>> {
    claims.ensure(organization_id, &[Permission::UpdatePriceList])?;
    Ok(Json(
        AttributeTypesRepo
            .update(&mut client, organization_id, &ref_, attribute_type)
            .await?,
    ))
}

/// Upsert PriceList
async fn upsert_attribute_type(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(mut client): PoolClient,
    ref_: Ref<AttributeType>,
    Json(attribute_type): Json<CreateAttributeType>,
) -> Result<(StatusCode, Json<AttributeType>)> {
    claims.ensure(
        organization_id,
        &[Permission::CreatePriceList, Permission::UpdatePriceList],
    )?;
    let (created, entity) = AttributeTypesRepo
        .upsert(
            &mut client,
            organization_id,
            claims.user_id(),
            &ref_,
            attribute_type,
        )
        .await?;
    if created {
        Ok((StatusCode::CREATED, Json(entity)))
    } else {
        Ok((StatusCode::OK, Json(entity)))
    }
}

/// Delete PriceList
async fn delete_attribute_type(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<AttributeType>,
    claims: Claims,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeletePriceList])?;
    AttributeTypesRepo
        .delete(&client, organization_id, &ref_)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
