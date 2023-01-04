use axum::extract::State;
use axum::{http::StatusCode, routing, Json};

use super::entities::Image;
use super::repo::ImagesRepo;
use crate::auth::signing::Claims;
use crate::auth::Permission;
use crate::cloudflare::CloudflareApi;
use crate::errors::Result;
use crate::extractors::PoolClient;
use crate::organizations::extractors::PathOrganizationId;
use crate::routes::AppRouter;
use crate::{Ref, UploadSourceImage};

pub(crate) fn org_routes() -> AppRouter {
    AppRouter::new().nest(
        "/images",
        AppRouter::new()
            .route("/", routing::get(list_images).post(upload_image))
            .route(
                "/:image_ref",
                routing::get(get_image)
                    .put(upsert_image)
                    .delete(delete_image),
            ),
    )
}

/// List Image items
async fn list_images(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    claims: Claims,
) -> Result<Json<Vec<Image>>> {
    claims.ensure(organization_id, &[Permission::ListImages])?;
    let images = ImagesRepo.list(&client, organization_id).await?;

    Ok(Json(images))
}

/// Get Image item
async fn get_image(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<Image>,
    claims: Claims,
) -> Result<Json<Image>> {
    claims.ensure(organization_id, &[Permission::GetImage])?;
    let res = ImagesRepo.find(&client, organization_id, &ref_).await?;
    if let Some(image) = res {
        Ok(Json(image))
    } else {
        Err(ref_.not_found_error())
    }
}

/// Create new Image
async fn upload_image(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(client): PoolClient,
    State(cloudflare_api): State<CloudflareApi>,
    Json(source_image): Json<UploadSourceImage>,
) -> Result<Json<Image>> {
    claims.ensure(organization_id, &[Permission::CreateImage])?;
    Ok(Json(
        ImagesRepo
            .insert(
                &client,
                &cloudflare_api,
                organization_id,
                claims.user_id(),
                source_image,
            )
            .await?,
    ))
}

/// Upsert Image
async fn upsert_image(
    PathOrganizationId(organization_id): PathOrganizationId,
    claims: Claims,
    PoolClient(client): PoolClient,
    State(cloudflare_api): State<CloudflareApi>,
    ref_: Ref<Image>,
    Json(source_image): Json<UploadSourceImage>,
) -> Result<(StatusCode, Json<Image>)> {
    claims.ensure(
        organization_id,
        &[Permission::CreateImage, Permission::UpdateImage],
    )?;
    let (created, entity) = ImagesRepo
        .upsert(
            &client,
            &cloudflare_api,
            organization_id,
            claims.user_id(),
            &ref_,
            source_image,
        )
        .await?;
    if created {
        Ok((StatusCode::CREATED, Json(entity)))
    } else {
        Ok((StatusCode::OK, Json(entity)))
    }
}

/// Delete Image
async fn delete_image(
    PathOrganizationId(organization_id): PathOrganizationId,
    PoolClient(client): PoolClient,
    ref_: Ref<Image>,
    claims: Claims,
) -> Result<StatusCode> {
    claims.ensure(organization_id, &[Permission::DeleteImage])?;
    ImagesRepo.delete(&client, organization_id, &ref_).await?;
    Ok(StatusCode::NO_CONTENT)
}
