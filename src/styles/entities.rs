use async_trait::async_trait;
use cornucopia_async::GenericClient;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::User,
    cornucopia::queries::style::get_style_id,
    entity_ref::{ExternalId, ExternalIdEntity, Id, RefTarget, Slug, SlugEntity},
    helpers::slugify,
    organizations::Organization,
    Attribute, AttributeSummary, Category, CategorySummary, EntityRefPathParam, I18nString,
    ImageSummary, NestedColor, NestedColorSummary, NestedPrice, NestedSize, Ref,
};

/// Style
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Style {
    pub id: Id<Self>,
    pub slug: Slug<Self>,
    pub number: String,
    pub name: I18nString,
    pub description: I18nString,
    pub core: Option<bool>,
    pub country_of_origin: Option<String>,
    pub tariff_no: Option<String>,
    #[serde(serialize_with = "rust_decimal::serde::float::serialize")]
    pub net_weight: Decimal,
    #[serde(serialize_with = "rust_decimal::serde::float::serialize")]
    pub gross_weight: Decimal,
    #[serde(serialize_with = "rust_decimal::serde::float::serialize")]
    pub unit_volume: Decimal,
    pub categories: Vec<Category>,
    pub attributes: Vec<AttributeSummary>,
    pub external_id: Option<ExternalId<Self>>,
    pub created_by: Option<Id<User>>,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
}

/// Style nested with colors and sizes, with some metadata fields excluded
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct NestedStyle {
    pub id: Id<Style>,
    pub slug: Slug<Style>,
    pub number: String,
    pub name: I18nString,
    pub description: I18nString,
    pub core: Option<bool>,
    pub country_of_origin: Option<String>,
    pub tariff_no: Option<String>,
    #[serde(serialize_with = "rust_decimal::serde::float::serialize")]
    pub net_weight: Decimal,
    #[serde(serialize_with = "rust_decimal::serde::float::serialize")]
    pub gross_weight: Decimal,
    #[serde(serialize_with = "rust_decimal::serde::float::serialize")]
    pub unit_volume: Decimal,
    pub categories: Vec<CategorySummary>,
    pub external_id: Option<ExternalId<Style>>,
    pub colors: Vec<NestedColor>,
    pub prices: Vec<NestedPrice>,
    pub attributes: Vec<AttributeSummary>,
    pub is_new: Option<bool>,
}

/// Style nested with colors and sizes, with some metadata fields excluded
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct NestedStyleSummary {
    pub id: Id<Style>,
    pub number: String,
    pub name: I18nString,
    pub colors: Vec<NestedColorSummary>,
}

impl NestedStyle {
    pub fn into_summary(self) -> NestedStyleSummary {
        NestedStyleSummary {
            id: self.id,
            number: self.number,
            name: self.name,
            colors: self.colors.into_iter().map(|c| c.into_summary()).collect(),
        }
    }

    pub fn primary_image(&self) -> Option<ImageSummary> {
        self.colors
            .first()
            .and_then(|color| color.primary_image())
            .cloned()
    }

    pub(crate) fn service_item(&self) -> bool {
        self.colors.iter().any(|color| color.service_item())
    }

    pub(crate) fn sizes(&self) -> Vec<&NestedSize> {
        self.colors
            .iter()
            .flat_map(|color| color.sizes.iter())
            .collect()
    }

    pub(crate) fn has_new_color(&self) -> bool {
        self.colors.iter().any(|color| color.is_new == Some(true))
    }
}

/// Style summary
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct StyleSummary {
    pub id: Id<Style>,
    pub number: String,
    pub name: I18nString,
    pub slug: Slug<Style>,
    pub external_id: Option<ExternalId<Style>>,
}

impl EntityRefPathParam for Style {
    fn parameter_name() -> &'static str {
        "style_ref"
    }
}

#[async_trait]
impl RefTarget for Style {
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        let (id, external_id, slug) = ref_.to_owned().take_all_inner();
        Ok(get_style_id()
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

/// Style, for creation
#[derive(Debug, Deserialize, Default)]
pub struct CreateStyle {
    pub slug: Option<Slug<Style>>,
    pub number: String,
    pub name: I18nString,
    #[serde(default)]
    pub description: I18nString,
    pub core: Option<bool>,
    pub country_of_origin: Option<String>,
    pub tariff_no: Option<String>,
    pub external_id: Option<ExternalId<Style>>,
    #[serde(default)]
    pub net_weight: Decimal,
    #[serde(default)]
    pub gross_weight: Decimal,
    #[serde(default)]
    pub unit_volume: Decimal,
    #[serde(default)]
    pub categories: Vec<Ref<Category>>,
    #[serde(default)]
    pub attributes: Vec<Ref<Attribute>>,
}

impl SlugEntity for CreateStyle {
    type RefTarget = Style;
    fn generate_slug(&self, prefix: &str) -> Option<Slug<Self::RefTarget>> {
        Some(Slug::new(slugify(&[prefix, &self.number, &self.name.en])))
    }

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for CreateStyle {
    type RefTarget = Style;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

/// Style, for update
#[derive(Debug, Deserialize, Default)]
pub struct UpdateStyle {
    pub external_id: Option<ExternalId<Style>>,
    pub slug: Option<Slug<Style>>,
    pub number: Option<String>,
    pub name: Option<I18nString>,
    pub description: Option<I18nString>,
    pub core: Option<bool>,
    pub country_of_origin: Option<String>,
    pub tariff_no: Option<String>,
    pub net_weight: Option<Decimal>,
    pub gross_weight: Option<Decimal>,
    pub unit_volume: Option<Decimal>,
    pub categories: Option<Vec<Ref<Category>>>,
    pub attributes: Option<Vec<Ref<Attribute>>>,
}

impl From<CreateStyle> for UpdateStyle {
    fn from(style: CreateStyle) -> Self {
        Self {
            number: Some(style.number),
            name: Some(style.name),
            slug: style.slug,
            external_id: style.external_id,
            description: Some(style.description),
            core: style.core,
            country_of_origin: style.country_of_origin,
            tariff_no: style.tariff_no,
            net_weight: Some(style.net_weight),
            gross_weight: Some(style.gross_weight),
            unit_volume: Some(style.unit_volume),
            categories: if style.categories.is_empty() {
                None
            } else {
                Some(style.categories)
            },
            attributes: if style.attributes.is_empty() {
                None
            } else {
                Some(style.attributes)
            },
        }
    }
}

impl SlugEntity for UpdateStyle {
    type RefTarget = Style;

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for UpdateStyle {
    type RefTarget = Style;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}
