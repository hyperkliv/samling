use async_trait::async_trait;
use cornucopi_async::GenericClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::User,
    cornucopia::queries::color::get_color_id,
    entity_ref::{ExternalId, ExternalIdEntity, Id, Ref, RefTarget, Slug, SlugEntity},
    helpers::slugify,
    organizations::Organization,
    EntityRefPathParam, I18nString, ImageSummary, NestedSize, NestedSizeSummary, Style,
    StyleSummary,
};

/// Color
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Color {
    pub id: Id<Self>,
    pub style: StyleSummary,
    pub number: String,
    pub name: I18nString,
    pub slug: Slug<Self>,
    pub images: Vec<ImageSummary>,
    pub external_id: Option<ExternalId<Self>>,
    pub created_by: Option<Id<User>>,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
}

impl EntityRefPathParam for Color {
    fn parameter_name() -> &'static str {
        "color_ref"
    }
}

/// Color with sizes included
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct NestedColor {
    pub id: Id<Color>,
    pub number: String,
    pub name: I18nString,
    pub slug: Slug<Color>,
    pub external_id: Option<ExternalId<Color>>,
    pub images: Vec<ImageSummary>,
    pub sizes: Vec<NestedSize>,
    pub is_new: Option<bool>,
}

/// Color with sizes included
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct NestedColorSummary {
    pub id: Id<Color>,
    pub number: String,
    pub name: I18nString,
    pub primary_image: Option<ImageSummary>,
    pub sizes: Vec<NestedSizeSummary>,
}
impl NestedColor {
    pub(crate) fn into_summary(self) -> NestedColorSummary {
        NestedColorSummary {
            id: self.id,
            number: self.number,
            name: self.name,
            primary_image: self.images.into_iter().next(),
            sizes: self.sizes.into_iter().map(|c| c.into_summary()).collect(),
        }
    }

    pub(crate) fn service_item(&self) -> bool {
        self.sizes
            .iter()
            .any(|size| size.service_item == Some(true))
    }

    pub(crate) fn primary_image(&self) -> Option<&ImageSummary> {
        self.images.first()
    }
}

/// Color summary
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ColorSummary {
    pub id: Id<Color>,
    pub style: StyleSummary,
    pub number: String,
    pub name: I18nString,
    pub slug: Slug<Color>,
    pub external_id: Option<ExternalId<Color>>,
}

#[async_trait]
impl RefTarget for Color {
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        let (id, external_id, slug) = entity_ref.to_owned().take_all_inner();
        Ok(get_color_id()
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

/// Color, for creation
#[derive(Debug, Deserialize)]
pub struct CreateColor {
    pub style: Ref<Style>,
    pub number: String,
    pub name: I18nString,
    pub slug: Option<Slug<Color>>,
    pub external_id: Option<ExternalId<Color>>,
}

impl SlugEntity for CreateColor {
    type RefTarget = Color;
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

impl ExternalIdEntity for CreateColor {
    type RefTarget = Color;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

/// Color, for update
#[derive(Debug, Deserialize)]
pub struct UpdateColor {
    pub style: Option<Ref<Style>>,
    pub number: Option<String>,
    pub name: Option<I18nString>,
    pub slug: Option<Slug<Color>>,
    pub external_id: Option<ExternalId<Color>>,
}

impl From<CreateColor> for UpdateColor {
    fn from(color: CreateColor) -> Self {
        Self {
            style: Some(color.style),
            number: Some(color.number),
            name: Some(color.name),
            slug: color.slug,
            external_id: color.external_id,
        }
    }
}

impl SlugEntity for UpdateColor {
    type RefTarget = Color;

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for UpdateColor {
    type RefTarget = Color;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}
