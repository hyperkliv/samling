use cornucopi_async::{GenericClient, Params};

use super::{CreateSize, Size, UpdateSize};
use crate::auth::User;
use crate::cornucopia::queries::size::{
    delete_size, get_size, insert_size, list_sizes, update_size, DeleteSizeParams,
    InsertSizeParams, SizeRow, SizeRowBorrowed, UpdateSizeParams,
};
use crate::entity_ref::{ExternalIdEntity, Id, Ref, RefTarget, SlugEntity};
use crate::organizations::Organization;
use crate::{Color, ColorsRepo, Result};

#[derive(Clone)]
pub struct SizesRepo;

impl SizesRepo {
    pub async fn list(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Vec<Size>> {
        list_sizes()
            // TODO: Implement pagination
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
        ref_: &Ref<Size>,
    ) -> Result<Option<Size>> {
        let (id, external_id, slug) = ref_.to_owned().take_all_inner();
        if let Some(res) = get_size()
            .bind(
                client,
                &organization_id.into(),
                &id,
                &external_id.as_deref(),
                &slug.as_deref(),
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
        ref_: &Ref<Size>,
    ) -> Result<Size> {
        if let Some(size) = self.find(client, organization_id, ref_).await? {
            Ok(size)
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub async fn insert(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        size: CreateSize,
    ) -> Result<Size> {
        let (color_id, _, color_slug) =
            ColorsRepo::get_refs(client, organization_id, &size.color).await?;
        let slug = size
            .check_or_generate_slug(client, organization_id, &color_slug)
            .await?;
        size.ensure_available_external_id(client, organization_id)
            .await?;
        let inserted_id: Id<Size> = insert_size()
            .params(
                client,
                &InsertSizeParams {
                    color_id: color_id.into(),
                    organization_id: organization_id.into(),
                    slug: slug.as_ref(),
                    external_id: size.external_id.as_deref(),
                    number: size.number.as_str(),
                    name: size.name,
                    service_item: size.service_item,
                    delivery_period: size.delivery_period,
                    ean_code: size.ean_code,
                    status: size.status,
                    created_by: created_by.into(),
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
        organization_id: Id<Organization>,
        ref_: &Ref<Size>,
        size: UpdateSize,
    ) -> crate::Result<Size> {
        if let Some(existing) = self.find(client, organization_id, ref_).await? {
            if size.slug.is_some() && Some(existing.slug) != size.slug {
                size.ensure_available_slug(client, organization_id).await?;
            }
            if size.external_id.is_some() && existing.external_id != size.external_id {
                size.ensure_available_external_id(client, organization_id)
                    .await?;
            }
            let color_id = if let Some(color_ref) = size.color {
                Color::get_id(client, organization_id, &color_ref).await?
            } else {
                existing.color.id
            };
            let num_updated = update_size()
                .params(
                    client,
                    &UpdateSizeParams {
                        color_id: color_id.into(),
                        slug: size.slug.as_deref(),
                        external_id: size.external_id.as_deref(),
                        number: size.number.as_deref(),
                        position: size.position.map(|p| p as i16),
                        name: size.name,
                        service_item: size.service_item,
                        delivery_period: size.delivery_period,
                        ean_code: size.ean_code,
                        status: size.status,
                        id: existing.id.into(),
                    },
                )
                .await?;
            debug_assert_eq!(num_updated, 1);
            self.get(client, organization_id, &existing.id.into()).await
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub async fn upsert(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        size_ref: &Ref<Size>,
        mut size: CreateSize,
    ) -> Result<(bool, Size)> {
        size.update_slug_from_ref(size_ref);
        size.update_external_id_from_ref(size_ref);
        if Size::lookup_id(client, organization_id, size_ref)
            .await?
            .is_some()
        {
            Ok((
                false,
                self.update(client, organization_id, size_ref, size.into())
                    .await?,
            ))
        } else {
            Ok((
                true,
                self.insert(client, organization_id, created_by, size)
                    .await?,
            ))
        }
    }

    pub async fn delete(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Size>,
    ) -> Result<()> {
        let id = Size::get_id(client, organization_id, ref_).await?;
        let num_deleted = delete_size()
            .params(
                client,
                &DeleteSizeParams {
                    organization_id: organization_id.into(),
                    id: id.into(),
                },
            )
            .await?;
        debug_assert_eq!(num_deleted, 1);
        Ok(())
    }
}

fn handle_row(row: SizeRowBorrowed) -> Result<Size> {
    let row: SizeRow = row.into();
    let color = serde_path_to_error::deserialize(row.color)?;
    Ok(Size {
        id: row.id.into(),
        color,
        name: serde_path_to_error::deserialize(row.name)?,
        number: row.number,
        position: row.position as u8,
        service_item: row.service_item,
        delivery_period: row.delivery_period,
        ean_code: row.ean_code,
        status: row.status,
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
        updated_at: row.updated_at,
        created_at: row.created_at,
        created_by: row.created_by.map(|v| v.into()),
    })
}
