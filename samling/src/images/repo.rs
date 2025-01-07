use samling_clorinde::{
    client::{GenericClient, Params},
    queries::image::{
        delete_image, get_image, get_image_url_by_external_checksum, insert_image, list_images,
        update_image, GetImageUrlByExternalChecksumParams, ImageRow, ImageRowBorrowed,
        InsertImageParams, UpdateImageParams,
    },
};

use super::Image;
use crate::{
    auth::User,
    entity_ref::{ExternalIdEntity, Id, RefTarget},
    organizations::Organization,
    CloudflareApi, ColorsRepo, Ref, Result, UploadSourceImage, UploadedImage,
};

#[derive(Clone)]
pub struct ImagesRepo;

impl ImagesRepo {
    pub async fn list(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Vec<Image>> {
        // TODO: Implement pagination
        list_images()
            .bind(client, &organization_id.into())
            .map(handle_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()
    }

    pub async fn find(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        image_ref: &Ref<Image>,
    ) -> Result<Option<Image>> {
        let (id, external_id, _) = image_ref.to_owned().take_all_inner();
        if let Some(res) = get_image()
            .bind(
                client,
                &organization_id.into(),
                &id,
                &external_id.as_deref(),
            )
            .map(handle_row)
            .opt()
            .await?
        {
            Ok(Some(res?))
        } else {
            Ok(None)
        }
    }

    pub async fn get(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Image>,
    ) -> Result<Image> {
        if let Some(image) = self.find(client, organization_id, ref_).await? {
            Ok(image)
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub async fn insert(
        &self,
        client: &impl GenericClient,
        cloudflare_api: &CloudflareApi,
        organization_id: Id<Organization>,
        uploaded_by: Id<User>,
        source_image: UploadSourceImage,
    ) -> Result<Image> {
        source_image
            .ensure_available_external_id(client, organization_id)
            .await?;
        let (color_id, _, color_slug) =
            ColorsRepo::get_refs(client, organization_id, &source_image.color).await?;
        let maybe_uploaded_url =
            if let Some(external_checksum) = source_image.external_checksum.as_ref() {
                get_image_url_by_external_checksum()
                    .params(
                        client,
                        &GetImageUrlByExternalChecksumParams {
                            organization_id: organization_id.into(),
                            external_checksum,
                        },
                    )
                    .map(|value| url::Url::parse(value).unwrap())
                    .opt()
                    .await?
            } else {
                None
            };
        let uploaded_url = if let Some(url) = maybe_uploaded_url {
            url
        } else {
            cloudflare_api
                .upload_image(
                    [
                        color_slug.to_string(),
                        source_image
                            .position
                            .map(|pos| format!("-{pos}"))
                            .unwrap_or_default(),
                    ]
                    .join(""),
                    source_image.source_url.into(),
                )
                .await?
        };
        let image = UploadedImage {
            url: uploaded_url,
            color: source_image.color,
            external_id: source_image.external_id,
        };
        let inserted_id: Id<Image> = insert_image()
            .params(
                client,
                &InsertImageParams {
                    color_id: color_id.into(),
                    external_id: image.external_id.as_deref(),
                    organization_id: organization_id.into(),
                    uploaded_by: uploaded_by.into(),
                    url: image.url.to_string(),
                    external_checksum: source_image.external_checksum,
                    position: source_image.position.unwrap_or_default(),
                },
            )
            .one()
            .await?
            .into();
        self.get(client, organization_id, &inserted_id.into()).await
    }

    pub async fn update(
        &self,
        client: &impl GenericClient,
        cloudflare_api: &CloudflareApi,
        organization_id: Id<Organization>,
        image_ref: &Ref<Image>,
        source_image: UploadSourceImage,
    ) -> Result<Image> {
        if let Some(existing) = self.find(client, organization_id, image_ref).await? {
            let (color_id, _, color_slug) =
                ColorsRepo::get_refs(client, organization_id, &source_image.color).await?;
            let uploaded_url = if existing.external_checksum.is_some()
                && existing.external_checksum == source_image.external_checksum
            {
                existing.url
            } else {
                cloudflare_api
                    .upload_image(
                        [
                            color_slug.to_string(),
                            source_image
                                .position
                                .map(|pos| format!("-{pos}"))
                                .unwrap_or_default(),
                        ]
                        .join(""),
                        source_image.source_url.into(),
                    )
                    .await?
            };
            let image = UploadedImage {
                url: uploaded_url,
                color: source_image.color,
                external_id: source_image.external_id,
            };
            if image.external_id.is_some() && existing.external_id != image.external_id {
                image
                    .ensure_available_external_id(client, organization_id)
                    .await?;
            }
            let num_updated = update_image()
                .params(
                    client,
                    &UpdateImageParams {
                        external_id: image.external_id.as_deref(),
                        id: existing.id.into(),
                        color_id: color_id.into(),
                        url: Some(image.url.to_string()),
                        external_checksum: source_image.external_checksum,
                        position: source_image.position,
                    },
                )
                .await?;
            debug_assert_eq!(num_updated, 1);
            self.get(client, organization_id, &existing.id.into()).await
        } else {
            Err(image_ref.not_found_error())
        }
    }

    pub async fn upsert(
        &self,
        client: &impl GenericClient,
        cloudflare_api: &CloudflareApi,
        organization_id: Id<Organization>,
        uploaded_by: Id<User>,
        image_ref: &Ref<Image>,
        mut source_image: UploadSourceImage,
    ) -> Result<(bool, Image)> {
        source_image.update_external_id_from_ref(image_ref);
        if Image::lookup_id(client, organization_id, image_ref)
            .await?
            .is_some()
        {
            Ok((
                false,
                self.update(
                    client,
                    cloudflare_api,
                    organization_id,
                    image_ref,
                    source_image,
                )
                .await?,
            ))
        } else {
            Ok((
                true,
                self.insert(
                    client,
                    cloudflare_api,
                    organization_id,
                    uploaded_by,
                    source_image,
                )
                .await?,
            ))
        }
    }

    pub async fn delete(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        image_ref: &Ref<Image>,
    ) -> Result<()> {
        let id = Image::get_id(client, organization_id, image_ref).await?;
        let num_deleted = delete_image()
            .bind(client, &organization_id.into(), (&id).into())
            .await?;
        debug_assert_eq!(num_deleted, 1);
        Ok(())
    }
}

fn handle_row(row: ImageRowBorrowed) -> Result<Image> {
    let row: ImageRow = row.into();
    let color = serde_path_to_error::deserialize(row.color)?;
    Ok(Image {
        id: row.id.into(),
        external_id: row.external_id.map(|v| v.into()),
        external_checksum: row.external_checksum,
        updated_at: row.updated_at,
        uploaded_at: row.uploaded_at,
        uploaded_by: row.uploaded_by.map(|v| v.into()),
        color,
        url: row.url.parse().unwrap(),
        position: row.position,
    })
}
