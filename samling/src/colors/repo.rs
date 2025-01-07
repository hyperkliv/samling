use samling_clorinde::client::{GenericClient, Params};

use crate::auth::User;
use crate::{
    Color, CreateColor, ExternalId, ExternalIdEntity, Id, Organization, Ref, RefTarget, Result,
    Slug, SlugEntity, Style, StylesRepo, UpdateColor,
};
use samling_clorinde::queries::color::{
    delete_color, get_color, get_color_refs, insert_color, list_colors, update_color, ColorRow,
    ColorRowBorrowed, DeleteColorParams, InsertColorParams, UpdateColorParams,
};

#[derive(Clone)]
pub struct ColorsRepo;

impl ColorsRepo {
    pub async fn list(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Vec<Color>> {
        list_colors()
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
        ref_: &Ref<Color>,
    ) -> Result<Option<Color>> {
        let (id, external_id, slug) = ref_.to_owned().take_all_inner();
        if let Some(res) = get_color()
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
        ref_: &Ref<Color>,
    ) -> Result<Color> {
        if let Some(color) = self.find(client, organization_id, ref_).await? {
            Ok(color)
        } else {
            Err(ref_.not_found_error())
        }
    }

    /// Get all reference values for the given input ref (e.g. an Id, ExternalId or Slug)
    /// or return a NotFound error.
    pub(crate) async fn get_refs(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Color>,
    ) -> Result<(Id<Style>, Option<ExternalId<Style>>, Slug<Style>)> {
        let (id, external_id, slug) = ref_.to_owned().take_all_inner();
        if let Some(row) = get_color_refs()
            .bind(
                client,
                &organization_id.into(),
                &id,
                &external_id.as_deref(),
                &slug.as_deref(),
            )
            .opt()
            .await?
        {
            Ok((
                row.id.into(),
                row.external_id.map(|v| v.into()),
                row.slug.into(),
            ))
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub async fn insert(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        color: CreateColor,
    ) -> Result<Color> {
        let (style_id, _, style_slug) =
            StylesRepo::get_refs(client, organization_id, &color.style).await?;
        let slug = color
            .check_or_generate_slug(client, organization_id, &style_slug)
            .await?;
        color
            .ensure_available_external_id(client, organization_id)
            .await?;
        let inserted_id: Id<Color> = insert_color()
            .params(
                client,
                &InsertColorParams {
                    style_id: style_id.into(),
                    organization_id: organization_id.into(),
                    slug: slug.as_ref(),
                    external_id: color.external_id.as_deref(),
                    number: color.number.as_str(),
                    name: color.name,
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
        ref_: &Ref<Color>,
        color: UpdateColor,
    ) -> crate::Result<Color> {
        if let Some(existing) = self.find(client, organization_id, ref_).await? {
            if color.slug.is_some() && Some(existing.slug) != color.slug {
                color.ensure_available_slug(client, organization_id).await?;
            }
            if color.external_id.is_some() && existing.external_id != color.external_id {
                color
                    .ensure_available_external_id(client, organization_id)
                    .await?;
            }
            let style_id = if let Some(style_ref) = color.style {
                Style::get_id(client, organization_id, &style_ref).await?
            } else {
                existing.style.id
            };
            let num_affected = update_color()
                .params(
                    client,
                    &UpdateColorParams {
                        style_id: style_id.into(),
                        slug: color.slug.as_deref(),
                        external_id: color.external_id.as_deref(),
                        number: color.number.as_deref(),
                        name: color.name,
                        id: existing.id.into(),
                    },
                )
                .await?;
            debug_assert_eq!(num_affected, 1);
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
        color_ref: &Ref<Color>,
        mut color: CreateColor,
    ) -> Result<(bool, Color)> {
        color.update_slug_from_ref(color_ref);
        color.update_external_id_from_ref(color_ref);
        if Color::lookup_id(client, organization_id, color_ref)
            .await?
            .is_some()
        {
            Ok((
                false,
                self.update(client, organization_id, color_ref, color.into())
                    .await?,
            ))
        } else {
            Ok((
                true,
                self.insert(client, organization_id, created_by, color)
                    .await?,
            ))
        }
    }

    pub async fn delete(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Color>,
    ) -> Result<()> {
        let id = Color::get_id(client, organization_id, ref_).await?;
        let num_deleted = delete_color()
            .params(
                client,
                &DeleteColorParams {
                    organization_id: organization_id.into(),
                    id: id.into(),
                },
            )
            .await?;
        debug_assert_eq!(num_deleted, 1);
        Ok(())
    }
}

fn handle_row(row: ColorRowBorrowed) -> Result<Color> {
    let row: ColorRow = row.into();
    let style = serde_path_to_error::deserialize(row.style)?;
    Ok(Color {
        id: row.id.into(),
        style,
        name: serde_path_to_error::deserialize(row.name)?,
        number: row.number,
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
        updated_at: row.updated_at,
        created_at: row.created_at,
        created_by: row.created_by.map(|v| v.into()),
        images: serde_path_to_error::deserialize(row.images)?,
    })
}
