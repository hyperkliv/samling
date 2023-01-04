use cornucopia_async::{GenericClient, Params};
use deadpool_postgres::Object;
use futures::{future::try_join_all, try_join};
use itertools::Itertools;

use super::{Collection, CreateCollection, UpdateCollection};
use crate::{
    cornucopia::{
        queries::{
            collection::{
                associate_collection_sizes, delete_collection, insert_collection,
                replace_collection_pricelists, select_collection_summaries, select_collections,
                set_new_collection_colors, set_new_collection_styles, update_collection,
                AssociateCollectionSizesParams, CollectionRow, CollectionRowBorrowed,
                CollectionSummaryRow, CollectionSummaryRowBorrowed, InsertCollectionParams,
                ReplaceCollectionPricelistsParams, SelectCollectionSummariesParams,
                SelectCollectionsParams, SetNewCollectionColorsParams,
                SetNewCollectionStylesParams, UpdateCollectionParams,
            },
            group::ensure_superuser_access,
        },
        types::public::CollectionPricelistRelation,
    },
    entity_ref::{ExternalIdEntity, Id, RefTarget, SlugEntity},
    organizations::Organization,
    CloudflareApi, CollectionItem, CollectionPricing, CollectionSummary, CollectionWithItems,
    Color, NestedStyleSortOrder, Ref, RequestMetaData, ResolvedCollectionFilters, Result, Size,
    Style, StylesRepo,
};

#[derive(Clone)]
pub struct CollectionsRepo;

impl CollectionsRepo {
    pub async fn list(
        &self,
        client: &impl GenericClient,
        metadata: RequestMetaData,
    ) -> Result<Vec<Collection>> {
        select_collections()
            .params(
                client,
                &SelectCollectionsParams {
                    organization_id: metadata.organization_id().into(),
                    requester_id: metadata.user_id().into(),
                    id: None,
                    external_id: None::<&str>,
                    slug: None::<&str>,
                },
            )
            .map(handle_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()
    }

    pub async fn list_summaries(
        &self,
        client: &impl GenericClient,
        metadata: RequestMetaData,
    ) -> Result<Vec<CollectionSummary>> {
        select_collection_summaries()
            .params(
                client,
                &SelectCollectionSummariesParams {
                    organization_id: metadata.organization_id().into(),
                    requester_id: metadata.user_id().into(),
                    id: None,
                    external_id: None::<&str>,
                    slug: None::<&str>,
                },
            )
            .map(handle_summary_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()
    }

    pub async fn find(
        &self,
        client: &impl GenericClient,
        metadata: RequestMetaData,
        collection_ref: &Ref<Collection>,
    ) -> Result<Option<Collection>> {
        let (id, external_id, slug) = collection_ref.to_owned().take_all_inner();
        if let Some(res) = select_collections()
            .params(
                client,
                &SelectCollectionsParams {
                    organization_id: metadata.organization_id().into(),
                    requester_id: metadata.user_id().into(),
                    id,
                    external_id,
                    slug,
                },
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
        metadata: RequestMetaData,
        ref_: &Ref<Collection>,
    ) -> Result<Collection> {
        if let Some(collection) = self.find(client, metadata, ref_).await? {
            Ok(collection)
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub(crate) async fn get_with_items(
        &self,
        client: &impl GenericClient,
        metadata: RequestMetaData,
        ref_: &Ref<Collection>,
        filters: ResolvedCollectionFilters,
        sorter: NestedStyleSortOrder,
    ) -> Result<CollectionWithItems> {
        let collection = self.get_summary(client, metadata, ref_).await?;
        let (collection, nested_styles) = try_join!(
            self.get_summary(client, metadata, ref_),
            StylesRepo.list_collection_nested(
                client,
                metadata,
                collection.id,
                filters.styles,
                sorter
            ),
        )?;

        let collection_style_items = nested_styles
            .into_iter()
            .map(|style| CollectionItem {
                style,
                user_comment: Default::default(),
            })
            .collect_vec();
        Ok(collection.with_items(collection_style_items))
    }

    pub(crate) async fn get_collection_item(
        &self,
        client: &impl GenericClient,
        metadata: RequestMetaData,
        collection_ref: &Ref<Collection>,
        style_ref: &Ref<Style>,
    ) -> Result<CollectionItem> {
        let collection = self.get_summary(client, metadata, collection_ref).await?;
        let style = StylesRepo
            .get_nested(client, metadata, collection.id, style_ref)
            .await?;
        Ok(CollectionItem {
            style,
            user_comment: Default::default(),
        })
    }

    pub async fn get_summary(
        &self,
        client: &impl GenericClient,
        metadata: RequestMetaData,
        collection_ref: &Ref<Collection>,
    ) -> crate::Result<CollectionSummary> {
        let (id, external_id, slug) = collection_ref.to_owned().take_all_inner();
        if let Some(collection_summary) = select_collection_summaries()
            .params(
                client,
                &SelectCollectionSummariesParams {
                    organization_id: metadata.organization_id().into(),
                    requester_id: metadata.user_id().into(),
                    id,
                    external_id,
                    slug,
                },
            )
            .opt()
            .await?
        {
            Ok(collection_summary.try_into()?)
        } else {
            Err(collection_ref.not_found_error())
        }
    }

    pub async fn insert(
        &self,
        client: &mut Object,
        cloudflare_api: CloudflareApi,
        metadata: RequestMetaData,
        collection: CreateCollection,
    ) -> Result<Collection> {
        let organization_id = metadata.organization_id();
        let tx = client.transaction().await?;
        let slug = collection
            .check_or_generate_slug(&tx, organization_id, "")
            .await?;
        collection
            .ensure_available_external_id(&tx, organization_id)
            .await?;
        let image_url = if let Some(image) = collection.image {
            Some(
                cloudflare_api
                    .upload_image(slug.clone().into(), image)
                    .await?,
            )
        } else {
            None
        };
        let inserted_id: Id<Collection> = insert_collection()
            .params(
                &tx,
                &InsertCollectionParams {
                    acronym: collection.acronym,
                    name: collection.name,
                    image_url: image_url.map(|url| url.to_string()),
                    slug: slug.as_ref(),
                    external_id: collection.external_id.as_deref(),
                    organization_id: organization_id.into(),
                    created_by: metadata.user_id().into(),
                },
            )
            .one()
            .await?
            .into();

        self.replace_pricing(&tx, &metadata, &inserted_id, &collection.pricing)
            .await?;
        self.associate_sizes(&tx, organization_id, &inserted_id, &collection.sizes)
            .await?;
        self.associate_new_styles(&tx, organization_id, &inserted_id, &collection.new_styles)
            .await?;
        self.associate_new_colors(&tx, organization_id, &inserted_id, &collection.new_colors)
            .await?;
        tx.commit().await?;
        ensure_superuser_access()
            .bind(client, &organization_id.into())
            .one()
            .await?;
        self.get(client, metadata, &inserted_id.into()).await
    }

    pub async fn update(
        &self,
        client: &mut Object,
        cloudflare_api: CloudflareApi,
        metadata: RequestMetaData,
        collection_ref: &Ref<Collection>,
        collection: UpdateCollection,
    ) -> Result<Collection> {
        let organization_id = metadata.organization_id();
        if let Some(existing) = self.find(client, metadata, collection_ref).await? {
            let tx = client.transaction().await?;
            if collection.slug.is_some() && Some(&existing.slug) != collection.slug.as_ref() {
                collection
                    .ensure_available_slug(&tx, organization_id)
                    .await?;
            }
            if collection.external_id.is_some() && existing.external_id != collection.external_id {
                collection
                    .ensure_available_external_id(&tx, organization_id)
                    .await?;
            }
            let image_url = if let Some(image) = collection.image {
                Some(
                    cloudflare_api
                        .upload_image(
                            collection
                                .slug
                                .as_ref()
                                .unwrap_or(&existing.slug)
                                .to_owned()
                                .into(),
                            image,
                        )
                        .await?,
                )
            } else {
                None
            };
            let num_updated = update_collection()
                .params(
                    &tx,
                    &UpdateCollectionParams {
                        acronym: collection.acronym,
                        name: collection.name,
                        image_url: image_url.map(|url| url.to_string()),
                        slug: collection.slug.as_deref(),
                        external_id: collection.external_id.as_deref(),
                        id: existing.id.into(),
                    },
                )
                .await?;
            debug_assert_eq!(num_updated, 1);

            if let Some(pricing) = collection.pricing {
                self.replace_pricing(&tx, &metadata, &existing.id, &pricing)
                    .await?;
            }

            if let Some(sizes) = collection.sizes {
                self.associate_sizes(&tx, organization_id, &existing.id, &sizes)
                    .await?;
            }

            if let Some(new_styles) = collection.new_styles {
                self.associate_new_styles(&tx, organization_id, &existing.id, &new_styles)
                    .await?;
            }

            if let Some(new_colors) = collection.new_colors {
                self.associate_new_colors(&tx, organization_id, &existing.id, &new_colors)
                    .await?;
            }

            let collection = self.get(&tx, metadata, collection_ref).await?;
            tx.commit().await?;
            Ok(collection)
        } else {
            Err(collection_ref.not_found_error())
        }
    }

    pub async fn upsert(
        &self,
        client: &mut Object,
        cloudflare_api: CloudflareApi,
        metadata: RequestMetaData,
        collection_ref: &Ref<Collection>,
        mut collection: CreateCollection,
    ) -> Result<(bool, Collection)> {
        collection.update_slug_from_ref(collection_ref);
        collection.update_external_id_from_ref(collection_ref);
        if Collection::lookup_id(client, metadata.organization_id(), collection_ref)
            .await?
            .is_some()
        {
            Ok((
                false,
                self.update(
                    client,
                    cloudflare_api,
                    metadata,
                    collection_ref,
                    collection.to_owned().into(),
                )
                .await?,
            ))
        } else {
            Ok((
                true,
                self.insert(client, cloudflare_api, metadata, collection.to_owned())
                    .await?,
            ))
        }
    }

    pub async fn delete(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        collection_ref: &Ref<Collection>,
    ) -> Result<()> {
        let id = Collection::get_id(client, organization_id, collection_ref).await?;
        let num_deleted = delete_collection()
            .bind(client, &organization_id.into(), (&id).into())
            .await?;
        debug_assert_eq!(num_deleted, 1);
        Ok(())
    }

    pub(crate) async fn replace_pricing(
        &self,
        client: &impl GenericClient,
        metadata: &RequestMetaData,
        collection_id: &Id<Collection>,
        pricing: &[CollectionPricing],
    ) -> Result<()> {
        let collection_pricelist_relations = pricing
            .iter()
            .map(|cp| CollectionPricelistRelation {
                pricelist_id: cp.list.id.into(),
                price_date: cp.date,
                created_by: metadata.user_id().into(),
            })
            .unique_by(|rel| rel.pricelist_id) // TODO: Raise error to user instead
            .collect_vec();
        replace_collection_pricelists()
            .params(
                client,
                &ReplaceCollectionPricelistsParams {
                    collection_id: collection_id.into(),
                    collection_pricelist_relations,
                },
            )
            .all()
            .await?;
        Ok(())
    }

    pub(crate) async fn associate_sizes(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        collection_id: &Id<Collection>,
        sizes: &[Ref<Size>],
    ) -> Result<()> {
        let size_ids: Vec<i32> = try_join_all(
            sizes
                .iter()
                .map(|size_ref| Size::lookup_id(client, organization_id, size_ref)),
        )
        .await?
        .into_iter()
        .zip(sizes)
        .filter_map(|(id, size_ref)| {
            if id.is_none() {
                tracing::info!("{size_ref} doesn't exist in database");
                None
            } else {
                id
            }
        })
        .map(|id| id.into())
        .collect();

        associate_collection_sizes()
            .params(
                client,
                &AssociateCollectionSizesParams {
                    collection_id: collection_id.into(),
                    size_ids,
                },
            )
            .all()
            .await?;
        Ok(())
    }

    pub(crate) async fn associate_new_styles(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        collection_id: &Id<Collection>,
        styles: &[Ref<Style>],
    ) -> Result<()> {
        let style_ids: Vec<i32> = try_join_all(
            styles
                .iter()
                .map(|style_ref| Style::lookup_id(client, organization_id, style_ref)),
        )
        .await?
        .into_iter()
        .zip(styles)
        .filter_map(|(id, style_ref)| {
            if id.is_none() {
                tracing::info!("{style_ref} doesn't exist in database");
                None
            } else {
                id
            }
        })
        .map(|id| id.into())
        .collect();

        set_new_collection_styles()
            .params(
                client,
                &SetNewCollectionStylesParams {
                    collection_id: collection_id.into(),
                    style_ids,
                },
            )
            .all()
            .await?;
        Ok(())
    }

    pub(crate) async fn associate_new_colors(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        collection_id: &Id<Collection>,
        colors: &[Ref<Color>],
    ) -> Result<()> {
        let color_ids: Vec<i32> = try_join_all(
            colors
                .iter()
                .map(|ref_| Color::lookup_id(client, organization_id, ref_)),
        )
        .await?
        .into_iter()
        .zip(colors)
        .filter_map(|(id, ref_)| {
            if id.is_none() {
                tracing::info!("{ref_} doesn't exist in database");
                None
            } else {
                id
            }
        })
        .map(|id| id.into())
        .collect();

        set_new_collection_colors()
            .params(
                client,
                &SetNewCollectionColorsParams {
                    collection_id: collection_id.into(),
                    color_ids,
                },
            )
            .all()
            .await?;
        Ok(())
    }
}

fn handle_row(row: CollectionRowBorrowed) -> Result<Collection> {
    let row: CollectionRow = row.into();
    Ok(Collection {
        id: row.id.into(),
        acronym: serde_path_to_error::deserialize(row.acronym)?,
        name: serde_path_to_error::deserialize(row.name)?,
        pricing: serde_path_to_error::deserialize(row.pricing)?,
        image_url: row.image_url.map(|url| url.parse().unwrap()),
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
        updated_at: row.updated_at,
        created_at: row.created_at,
        created_by: row.created_by.map(|v| v.into()),
        sizes: serde_path_to_error::deserialize(row.sizes)?,
    })
}

fn handle_summary_row(row: CollectionSummaryRowBorrowed) -> Result<CollectionSummary> {
    let row: CollectionSummaryRow = row.into();
    Ok(CollectionSummary {
        id: row.id.into(),
        acronym: serde_path_to_error::deserialize(row.acronym)?,
        name: serde_path_to_error::deserialize(row.name)?,
        pricing: serde_path_to_error::deserialize(row.pricing)?,
        image_url: row.image_url.map(|url| url.parse().unwrap()),
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
        updated_at: row.updated_at,
        created_at: row.created_at,
        created_by: row.created_by.map(|v| v.into()),
        num_styles: row.num_styles as u32,
        num_colors: row.num_colors as u32,
        num_sizes: row.num_sizes as u32,
    })
}

impl TryFrom<CollectionSummaryRow> for CollectionSummary {
    type Error = crate::Error;
    fn try_from(row: CollectionSummaryRow) -> crate::Result<Self> {
        Ok(Self {
            id: row.id.into(),
            acronym: serde_path_to_error::deserialize(row.acronym)?,
            name: serde_path_to_error::deserialize(row.name)?,
            image_url: row.image_url.map(|url| url.parse().unwrap()),
            pricing: serde_path_to_error::deserialize(row.pricing)?,
            slug: row.slug.into(),
            external_id: row.external_id.map(|e| e.into()),
            updated_at: row.updated_at,
            created_at: row.created_at,
            created_by: row.created_by.map(|v| v.into()),
            num_styles: row.num_styles as u32,
            num_colors: row.num_colors as u32,
            num_sizes: row.num_sizes as u32,
        })
    }
}
