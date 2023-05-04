use cornucopia_async::{GenericClient, Params};
use deadpool_postgres::Object;
use futures::future::{try_join, try_join_all};
use itertools::Itertools;

use super::{CreateStyle, Style, UpdateStyle};
use crate::auth::User;
use crate::cornucopia::queries::attribute::{
    associate_style_attributes, AssociateStyleAttributesParams,
};
use crate::cornucopia::queries::category::{
    associate_style_categories, AssociateStyleCategoriesParams,
};
use crate::cornucopia::queries::pricelist::{allowed_pricelist_ids, AllowedPricelistIdsParams};
use crate::cornucopia::queries::style::{
    delete_style, get_style_refs, insert_style, select_collection_styles_nested,
    select_nested_style_summaries, select_styles, update_style, DeleteStyleParams,
    InsertStyleParams, NestedStyleRow, NestedStyleRowBorrowed, NestedStyleSummaryRow,
    NestedStyleSummaryRowBorrowed, SelectCollectionStylesNestedParams,
    SelectNestedStyleSummariesParams, SelectStylesParams, StyleRow, StyleRowBorrowed,
    UpdateStyleParams,
};
use crate::entity_ref::{ExternalIdEntity, Id, Ref, RefTarget, RefType, SlugEntity};
use crate::organizations::Organization;
use crate::sorting::Sortable;
use crate::{
    Attribute, Category, Collection, ExternalId, Filters, NestedStyle, NestedStyleSortOrder,
    NestedStyleSummary, PriceList, RequestMetaData, ResolvedFilters, ResolvedStyleFilters, Result,
    Slug, StyleFilters,
};

#[derive(Clone)]
pub struct StylesRepo;

impl StylesRepo {
    pub async fn list(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Vec<Style>> {
        select_styles()
            .params(
                client,
                &SelectStylesParams {
                    organization_id: organization_id.into(),
                    ids: None::<Vec<i32>>,
                },
            )
            .map(handle_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()
    }

    pub async fn list_collection_nested(
        &self,
        client: &impl GenericClient,
        metadata: RequestMetaData,
        collection_id: Id<Collection>,
        filters: ResolvedStyleFilters,
        sorter: NestedStyleSortOrder,
    ) -> Result<Vec<NestedStyle>> {
        let organization_id = metadata.organization_id();
        let ids = filters
            .ids
            .clone()
            .map(|vec| vec.iter().map(|id| id.take()).collect_vec());
        let pricelist_ids_to_display = filters
            .pricelist_ids
            .clone()
            .map(|vec| vec.iter().map(|id| id.take()).collect_vec());

        let mut allowed_pricelists_query = allowed_pricelist_ids();
        let allowed_pricelists_params = AllowedPricelistIdsParams {
            organization_id: metadata.organization_id().into(),
            user_id: metadata.user_id().into(),
        };
        let allowed_pricelists_query_task = allowed_pricelists_query
            .params(client, &allowed_pricelists_params)
            .map(Id::<PriceList>::new)
            .all();

        let mut styles_query = select_collection_styles_nested();
        let styles_query_params = SelectCollectionStylesNestedParams {
            organization_id: organization_id.into(),
            ids,
            collection_id: collection_id.into(),
            pricelist_ids_to_display,
            statuses: filters.status.clone(),
        };
        let styles_query_task = styles_query
            .params(client, &styles_query_params)
            .map(handle_nested_row)
            .all();

        let (styles, allowed_pricelists) =
            try_join(styles_query_task, allowed_pricelists_query_task).await?;

        let styles = styles.into_iter().collect::<Result<Vec<_>>>()?;

        let allowed_pricelists = {
            if let Some(pricelist_ids) = &filters.pricelist_ids {
                pricelist_ids
                    .iter()
                    .copied()
                    .filter(|id| allowed_pricelists.contains(id))
                    .collect()
            } else {
                allowed_pricelists
            }
        };
        let styles = styles
            .into_iter()
            .filter(|style| filters.keep(style))
            .map(|style| filters.remove_unauthorized_prices(style, &allowed_pricelists))
            .collect_vec();
        Ok(sorter.sort(styles))
    }

    pub async fn list_nested_summary(
        &self,
        client: &impl GenericClient,
        metadata: RequestMetaData,
        filters: ResolvedStyleFilters,
    ) -> Result<Vec<NestedStyleSummary>> {
        let organization_id = metadata.organization_id();

        let ids = filters
            .ids
            .clone()
            .map(|vec| vec.iter().map(|id| id.take()).collect_vec());
        let categories = filters
            .category_ids
            .clone()
            .map(|vec| vec.iter().map(|id| id.take()).collect_vec());
        let attributes = filters
            .attribute_ids
            .clone()
            .map(|vec| vec.iter().map(|id| id.take()).collect_vec());

        // TODO: Support all filters, not just status, ids, categories and attributes!
        let styles = select_nested_style_summaries()
            .params(
                client,
                &SelectNestedStyleSummariesParams {
                    organization_id: organization_id.into(),
                    statuses: filters.status,
                    ids,
                    categories,
                    attributes,
                },
            )
            .map(handle_nested_summary_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<Vec<_>>>()?;

        Ok(styles)
    }

    pub async fn find(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Style>,
    ) -> Result<Option<Style>> {
        if let Some(id) = Style::lookup_id(client, organization_id, ref_).await? {
            Ok(Some(
                select_styles()
                    .params(
                        client,
                        &SelectStylesParams {
                            organization_id: organization_id.into(),
                            ids: Some(vec![*id.inner()]),
                        },
                    )
                    .map(handle_row)
                    .one()
                    .await??,
            ))
        } else {
            Ok(None)
        }
    }

    pub async fn get(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Style>,
    ) -> Result<Style> {
        if let Some(style) = self.find(client, organization_id, ref_).await? {
            Ok(style)
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub async fn get_nested(
        &self,
        client: &impl GenericClient,
        metadata: RequestMetaData,
        collection_id: Id<Collection>,
        style_ref: &Ref<Style>,
    ) -> Result<NestedStyle> {
        let filters = StyleFilters::from_ref(style_ref);
        let filters = filters.resolve(client, metadata.organization_id()).await?;
        let mut styles = self
            .list_collection_nested(
                client,
                metadata,
                collection_id,
                filters,
                crate::NestedStyleSortOrder::default(),
            )
            .await?;
        match styles.len() {
            0 => Err(style_ref.not_found_error()),
            1 => Ok(styles.pop().unwrap()),
            _ => unreachable!("Shouldn't be possible tom get multiple styles!"),
        }
    }

    /// Get all Refs for the given input ref (e.g. an Id, ExternalId or Slug) or
    /// returns a NotFound error.
    pub(crate) async fn get_refs(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Style>,
    ) -> Result<(Id<Style>, Option<ExternalId<Style>>, Slug<Style>)> {
        let (id, external_id, slug) = ref_.to_owned().take_all_inner();
        if let Some(row) = get_style_refs()
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

    /// Create a new Style entity
    pub async fn insert(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        style: CreateStyle,
    ) -> Result<Style> {
        let tx = client.transaction().await?;
        let slug = style
            .check_or_generate_slug(&tx, organization_id, "")
            .await?;
        style
            .ensure_available_external_id(&tx, organization_id)
            .await?;
        let params = InsertStyleParams {
            organization_id: organization_id.take(),
            slug: slug.take(),
            external_id: style.external_id.as_deref(),
            number: style.number,
            name: style.name,
            description: style.description,
            core: style.core,
            country_of_origin: style.country_of_origin,
            tariff_no: style.tariff_no,
            net_weight: style.net_weight,
            gross_weight: style.gross_weight,
            unit_volume: style.unit_volume,
            created_by: created_by.take(),
        };
        let id: Id<Style> = insert_style().params(&tx, &params).one().await?.into();

        self.associate_categories(&tx, organization_id, id, style.categories)
            .await?;
        self.associate_attributes(&tx, organization_id, id, style.attributes)
            .await?;
        tx.commit().await?;
        self.get(client, organization_id, &id.into()).await
    }

    /// Do a partial update of a Style entity
    pub async fn update(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        ref_: &Ref<Style>,
        style: UpdateStyle,
    ) -> crate::Result<Style> {
        let tx = client.transaction().await?;
        if let Some(existing) = self.find(&tx, organization_id, ref_).await? {
            if style.slug.is_some() && Some(existing.slug) != style.slug {
                style.ensure_available_slug(&tx, organization_id).await?;
            }
            if style.external_id.is_some() && existing.external_id != style.external_id {
                style
                    .ensure_available_external_id(&tx, organization_id)
                    .await?;
            }

            update_style()
                .params(
                    &tx,
                    &UpdateStyleParams {
                        id: existing.id.into(),
                        external_id: style.external_id.as_deref(),
                        slug: style.slug.as_deref(),
                        number: style.number,
                        name: style.name,
                        description: style.description,
                        core: style.core,
                        country_of_origin: style.country_of_origin,
                        tariff_no: style.tariff_no,
                        net_weight: style.net_weight,
                        gross_weight: style.gross_weight,
                        unit_volume: style.unit_volume,
                    },
                )
                .one()
                .await?;

            if let Some(update_categories) = style.categories {
                self.associate_categories(&tx, organization_id, existing.id, update_categories)
                    .await?;
            }

            if let Some(update_attributes) = style.attributes {
                self.associate_attributes(&tx, organization_id, existing.id, update_attributes)
                    .await?;
            }
            tx.commit().await?;
            self.get(client, organization_id, &existing.id.into()).await
        } else {
            Err(ref_.not_found_error())
        }
    }

    /// Either create or update the given entity, depending on if the style reference can be found
    pub async fn upsert(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        style_ref: &Ref<Style>,
        mut style: CreateStyle,
    ) -> Result<(bool, Style)> {
        style.update_slug_from_ref(style_ref);
        style.update_external_id_from_ref(style_ref);
        if Style::lookup_id(client, organization_id, style_ref)
            .await?
            .is_some()
        {
            Ok((
                false,
                self.update(client, organization_id, style_ref, style.into())
                    .await?,
            ))
        } else {
            Ok((
                true,
                self.insert(client, organization_id, created_by, style)
                    .await?,
            ))
        }
    }

    /// Delete the given Style
    pub async fn delete(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Style>,
    ) -> Result<()> {
        let id = Style::get_id(client, organization_id, ref_).await?;
        let deleted_id = delete_style()
            .params(
                client,
                &DeleteStyleParams {
                    organization_id: organization_id.into(),
                    id: id.into(),
                },
            )
            .one()
            .await?;
        debug_assert_eq!(&deleted_id, id.inner());
        Ok(())
    }

    /// Associate the given categories to the style
    async fn associate_categories(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        style_id: Id<Style>,
        category_refs: Vec<Ref<Category>>,
    ) -> Result<()> {
        let mut category_ids: Vec<i32> = try_join_all(
            category_refs
                .iter()
                .map(|ref_| async { Category::get_id(client, organization_id, ref_).await }),
        )
        .await?
        .iter()
        .map(|v| v.into())
        .collect();

        category_ids.dedup();

        associate_style_categories()
            .params(
                client,
                &AssociateStyleCategoriesParams {
                    style_id: style_id.into(),
                    category_ids,
                },
            )
            .all()
            .await?;

        Ok(())
    }

    /// Associate the given attributes to the style
    async fn associate_attributes(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        style_id: Id<Style>,
        attribute_refs: Vec<Ref<Attribute>>,
    ) -> Result<()> {
        let mut attribute_ids: Vec<i32> = try_join_all(
            attribute_refs
                .iter()
                .map(|ref_| async { Attribute::get_id(client, organization_id, ref_).await }),
        )
        .await?
        .iter()
        .map(|v| v.into())
        .collect();

        attribute_ids.dedup();

        associate_style_attributes()
            .params(
                client,
                &AssociateStyleAttributesParams {
                    style_id: style_id.into(),
                    attribute_ids,
                },
            )
            .all()
            .await?;

        Ok(())
    }
}

fn handle_row(row: StyleRowBorrowed) -> Result<Style> {
    let row: StyleRow = row.into();
    let name = serde_path_to_error::deserialize(row.name)?;
    let description = serde_path_to_error::deserialize(row.description)?;
    let categories = serde_path_to_error::deserialize(row.categories)?;
    let attributes = serde_path_to_error::deserialize(row.attributes)?;
    Ok(Style {
        id: row.id.into(),
        name,
        number: row.number,
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
        created_by: row.created_by.map(|v| v.into()),
        updated_at: row.updated_at,
        created_at: row.created_at,
        description,
        core: row.core,
        country_of_origin: row.country_of_origin,
        tariff_no: row.tariff_no,
        net_weight: row.net_weight,
        gross_weight: row.gross_weight,
        unit_volume: row.unit_volume,
        categories,
        attributes,
    })
}

fn handle_nested_row(row: NestedStyleRowBorrowed) -> Result<NestedStyle> {
    let row: NestedStyleRow = row.into();
    Ok(NestedStyle {
        id: row.id.into(),
        name: serde_path_to_error::deserialize(row.name)?,
        number: row.number,
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
        description: serde_path_to_error::deserialize(row.description)?,
        core: row.core,
        country_of_origin: row.country_of_origin,
        tariff_no: row.tariff_no,
        net_weight: row.net_weight,
        gross_weight: row.gross_weight,
        unit_volume: row.unit_volume,
        categories: serde_path_to_error::deserialize(row.categories)?,
        colors: serde_path_to_error::deserialize(row.colors)?,
        prices: serde_path_to_error::deserialize(row.prices)?,
        attributes: serde_path_to_error::deserialize(row.attributes)?,
        is_new: row.is_new,
    })
}

fn handle_nested_summary_row(row: NestedStyleSummaryRowBorrowed) -> Result<NestedStyleSummary> {
    let row: NestedStyleSummaryRow = row.into();
    Ok(NestedStyleSummary {
        id: row.id.into(),
        name: serde_path_to_error::deserialize(row.name)?,
        number: row.number,
        colors: serde_path_to_error::deserialize(row.colors)?,
    })
}
