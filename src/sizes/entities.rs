use async_trait::async_trait;
use cornucopia_async::GenericClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::User,
    cornucopia::queries::size::get_size_id,
    entity_ref::{ExternalId, ExternalIdEntity, Id, Ref, RefTarget, Slug, SlugEntity},
    helpers::slugify,
    organizations::Organization,
    Color, ColorSummary, EntityRefPathParam, I18nString,
};

/// Size
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Size {
    pub id: Id<Size>,
    pub color: ColorSummary,
    pub number: String,
    pub position: u8,
    pub name: I18nString,
    pub service_item: Option<bool>,
    pub ean_code: Option<String>,
    pub status: Option<String>,
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date")]
    pub delivery_period: Option<time::Date>,
    pub slug: Slug<Size>,
    pub external_id: Option<ExternalId<Size>>,
    pub created_by: Option<Id<User>>,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
}

/// Nested size (well, used by NestedColor, so `color` field isn't needed)
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct NestedSize {
    pub id: Id<Size>,
    pub number: String,
    pub position: u8,
    pub name: I18nString,
    pub slug: Slug<Size>,
    pub external_id: Option<ExternalId<Size>>,
    pub service_item: Option<bool>,
    pub ean_code: Option<String>,
    pub status: Option<String>,
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date", default)]
    pub delivery_period: Option<time::Date>,
}

/// Nested size (well, used by NestedColor, so `color` field isn't needed)
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct NestedSizeSummary {
    pub id: Id<Size>,
    pub number: String,
    pub name: I18nString,
}

impl NestedSize {
    pub fn into_summary(self) -> NestedSizeSummary {
        NestedSizeSummary {
            id: self.id,
            number: self.number,
            name: self.name,
        }
    }
}

impl EntityRefPathParam for Size {
    fn parameter_name() -> &'static str {
        "size_ref"
    }
}

#[async_trait]
impl RefTarget for Size {
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        let (id, external_id, slug) = entity_ref.to_owned().take_all_inner();
        Ok(get_size_id()
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

/// Size, for creation
#[derive(Debug, Deserialize)]
pub struct CreateSize {
    pub color: Ref<Color>,
    pub number: String,
    pub position: u8,
    pub name: I18nString,
    pub service_item: Option<bool>,
    pub ean_code: Option<String>,
    pub status: Option<String>,
    pub delivery_period: Option<time::Date>,
    pub slug: Option<Slug<Size>>,
    pub external_id: Option<ExternalId<Size>>,
}

impl SlugEntity for CreateSize {
    type RefTarget = Size;
    fn generate_slug(&self, prefix: &str) -> Option<Slug<Self::RefTarget>> {
        Some(Slug::new(slugify(&[prefix, &self.number])))
    }

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for CreateSize {
    type RefTarget = Size;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

/// Size, for update
#[derive(Debug, Deserialize)]
pub struct UpdateSize {
    pub color: Option<Ref<Color>>,
    pub number: Option<String>,
    pub position: Option<u8>,
    pub name: Option<I18nString>,
    pub service_item: Option<bool>,
    pub delivery_period: Option<time::Date>,
    pub ean_code: Option<String>,
    pub status: Option<String>,
    pub slug: Option<Slug<Size>>,
    pub external_id: Option<ExternalId<Size>>,
}

impl From<CreateSize> for UpdateSize {
    fn from(size: CreateSize) -> Self {
        Self {
            color: Some(size.color),
            number: Some(size.number),
            position: Some(size.position),
            name: Some(size.name),
            service_item: size.service_item,
            delivery_period: size.delivery_period,
            ean_code: size.ean_code,
            status: size.status,
            slug: size.slug,
            external_id: size.external_id,
        }
    }
}

impl SlugEntity for UpdateSize {
    type RefTarget = Size;

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for UpdateSize {
    type RefTarget = Size;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}
