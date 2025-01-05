use async_trait::async_trait;
use cornucopi_async::GenericClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::User,
    cornucopia::queries::category::get_category_id,
    entity_ref::{ExternalId, ExternalIdEntity, Id, RefTarget, Slug, SlugEntity},
    helpers::slugify,
    organizations::Organization,
    EntityRefPathParam, I18nString,
};

/// Category
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Category {
    pub id: Id<Self>,
    pub slug: Slug<Self>,
    pub name: I18nString,
    pub external_id: Option<ExternalId<Self>>,
    pub created_by: Option<Id<User>>,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
}

/// Category summary
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CategorySummary {
    pub id: Id<Category>,
    pub slug: Slug<Category>,
    pub name: I18nString,
    pub external_id: Option<ExternalId<Category>>,
}

impl EntityRefPathParam for Category {
    fn parameter_name() -> &'static str {
        "category_ref"
    }
}

#[async_trait]
impl RefTarget for Category {
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        let (id, external_id, slug) = entity_ref.to_owned().take_all_inner();
        Ok(get_category_id()
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

/// Category, for creation
#[derive(Debug, Deserialize, Clone)]
pub struct CreateCategory {
    pub slug: Option<Slug<Category>>,
    pub name: I18nString,
    pub external_id: Option<ExternalId<Category>>,
}

impl SlugEntity for CreateCategory {
    type RefTarget = Category;
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

impl ExternalIdEntity for CreateCategory {
    type RefTarget = Category;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

/// Category, for update
#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub slug: Option<Slug<Category>>,
    pub name: Option<I18nString>,
    pub external_id: Option<ExternalId<Category>>,
}

impl From<CreateCategory> for UpdateCategory {
    fn from(data: CreateCategory) -> Self {
        Self {
            name: Some(data.name),
            slug: data.slug,
            external_id: data.external_id,
        }
    }
}

impl SlugEntity for UpdateCategory {
    type RefTarget = Category;

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for UpdateCategory {
    type RefTarget = Category;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}
