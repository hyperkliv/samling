use async_trait::async_trait;
use cornucopia_async::GenericClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    auth::User,
    cornucopia::queries::collection::get_collection_id,
    entity_ref::{ExternalId, ExternalIdEntity, Slug, SlugEntity},
    helpers::slugify,
    organizations::Organization,
    Color, EntityRefPathParam, I18nString, Id, ImageSource, NestedSize, NestedStyle,
    PriceListSummary, Ref, RefTarget, Size, Style,
};

impl EntityRefPathParam for Collection {
    fn parameter_name() -> &'static str {
        "collection_ref"
    }
}

/// Collection
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Collection {
    pub id: Id<Self>,
    pub acronym: I18nString,
    pub name: I18nString,
    pub image_url: Option<Url>,
    pub slug: Slug<Self>,
    pub external_id: Option<ExternalId<Self>>,
    pub created_by: Option<Id<User>>,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
    pub sizes: Vec<NestedSize>,
    pub pricing: Vec<CollectionPricing>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CollectionPricing {
    pub list: PriceListSummary,
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date")]
    pub date: time::Date,
}

/// Collection
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CollectionSummary {
    pub id: Id<Collection>,
    pub acronym: I18nString,
    pub name: I18nString,
    pub pricing: Vec<CollectionPricing>,
    pub image_url: Option<Url>,
    pub slug: Slug<Collection>,
    pub external_id: Option<ExternalId<Collection>>,
    pub created_by: Option<Id<User>>,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
    pub num_styles: u32,
    pub num_colors: u32,
    pub num_sizes: u32,
}

/// Collection
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CollectionWithItems {
    #[serde(flatten)]
    pub collection: CollectionSummary,
    pub items: Vec<CollectionItem>,
}

/// Collection
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CollectionItem {
    pub style: NestedStyle,
    pub user_comment: String, // TODO
}

#[async_trait]
impl RefTarget for Collection {
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        let (id, external_id, slug) = entity_ref.to_owned().take_all_inner();
        Ok(get_collection_id()
            .bind(
                client,
                &organization_id.into(),
                &id,
                &external_id.as_deref(),
                &slug.as_deref(),
            )
            .opt()
            .await?
            .map(Id::new))
    }
}

/// Collection, for creation
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CreateCollection {
    pub acronym: I18nString,
    pub name: I18nString,
    pub pricing: Vec<CollectionPricing>,
    pub image: Option<ImageSource>,
    pub slug: Option<Slug<Collection>>,
    pub external_id: Option<ExternalId<Collection>>,
    #[serde(default)]
    pub sizes: Vec<Ref<Size>>,
    pub new_styles: Vec<Ref<Style>>,
    pub new_colors: Vec<Ref<Color>>,
}

impl SlugEntity for CreateCollection {
    type RefTarget = Collection;
    fn generate_slug(&self, prefix: &str) -> Option<Slug<Self::RefTarget>> {
        Some(Slug::new(slugify(&[prefix, &self.name.en])))
    }

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for CreateCollection {
    type RefTarget = Collection;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

/// Collection, for update
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct UpdateCollection {
    pub acronym: Option<I18nString>,
    pub name: Option<I18nString>,
    pub pricing: Option<Vec<CollectionPricing>>,
    pub image: Option<ImageSource>,
    pub slug: Option<Slug<Collection>>,
    pub external_id: Option<ExternalId<Collection>>,
    pub sizes: Option<Vec<Ref<Size>>>,
    pub new_styles: Option<Vec<Ref<Style>>>,
    pub new_colors: Option<Vec<Ref<Color>>>,
}

impl From<CreateCollection> for UpdateCollection {
    fn from(collection: CreateCollection) -> Self {
        UpdateCollection {
            acronym: Some(collection.acronym),
            name: Some(collection.name),
            pricing: Some(collection.pricing),
            image: collection.image,
            slug: collection.slug,
            external_id: collection.external_id,
            sizes: Some(collection.sizes),
            new_styles: Some(collection.new_styles),
            new_colors: Some(collection.new_colors),
        }
    }
}

impl SlugEntity for UpdateCollection {
    type RefTarget = Collection;

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for UpdateCollection {
    type RefTarget = Collection;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}
impl CollectionSummary {
    pub(crate) fn with_items(self, items: Vec<CollectionItem>) -> CollectionWithItems {
        CollectionWithItems {
            collection: self,
            items,
        }
    }
}
