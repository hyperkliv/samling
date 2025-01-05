use async_trait::async_trait;
use cornucopi_async::GenericClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::User,
    cornucopia::queries::{attribute::get_attribute_id, attributetype::get_attribute_type_id},
    entity_ref::{ExternalId, ExternalIdEntity},
    helpers::slugify,
    organizations::Organization,
    EntityRefPathParam, I18nString, Id, Ref, RefTarget, Slug, SlugEntity,
};

/// Attribute type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AttributeType {
    pub id: Id<Self>,
    pub name: I18nString,
    pub slug: Slug<Self>,
    pub external_id: Option<ExternalId<Self>>,
    pub created_by: Option<Id<User>>,
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
}

/// Attribute type summary
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AttributeTypeSummary {
    pub id: Id<AttributeType>,
    pub name: I18nString,
    pub slug: Slug<AttributeType>,
    pub external_id: Option<ExternalId<AttributeType>>,
}

impl EntityRefPathParam for AttributeType {
    fn parameter_name() -> &'static str {
        "attribute_type_ref"
    }
}

#[async_trait]
impl RefTarget for AttributeType {
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        let (id, external_id, slug) = entity_ref.to_owned().take_all_inner();
        Ok(get_attribute_type_id()
            .bind(
                client,
                &organization_id.into(),
                &id,
                &external_id.as_deref(),
                &slug,
            )
            .opt()
            .await?
            .map(|id| id.into()))
    }
}

/// Attribute type, for creation
#[derive(Debug, Clone, Deserialize)]
pub struct CreateAttributeType {
    pub name: I18nString,
    pub slug: Option<Slug<AttributeType>>,
    pub external_id: Option<ExternalId<AttributeType>>,
}

impl ExternalIdEntity for CreateAttributeType {
    type RefTarget = AttributeType;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

impl SlugEntity for CreateAttributeType {
    type RefTarget = AttributeType;
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

/// Attribute type, for update
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAttributeType {
    pub name: Option<I18nString>,
    pub slug: Option<Slug<AttributeType>>,
    pub external_id: Option<ExternalId<AttributeType>>,
}

impl From<CreateAttributeType> for UpdateAttributeType {
    fn from(attribute_type: CreateAttributeType) -> Self {
        UpdateAttributeType {
            name: Some(attribute_type.name),
            slug: attribute_type.slug,
            external_id: attribute_type.external_id,
        }
    }
}

impl SlugEntity for UpdateAttributeType {
    type RefTarget = AttributeType;
    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for UpdateAttributeType {
    type RefTarget = AttributeType;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

/// Attribute
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Attribute {
    pub id: Id<Self>,
    pub r#type: AttributeTypeSummary,
    pub title: I18nString,
    pub description: I18nString,
    pub slug: Slug<Self>,
    pub external_id: Option<ExternalId<Self>>,
    pub created_by: Option<Id<User>>,
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
}

/// Nested attribute
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AttributeSummary {
    pub id: Id<Attribute>,
    pub r#type: AttributeTypeSummary,
    pub title: I18nString,
    pub description: I18nString,
    pub slug: Slug<Attribute>,
    pub external_id: Option<ExternalId<Attribute>>,
}

impl EntityRefPathParam for Attribute {
    fn parameter_name() -> &'static str {
        "attribute_ref"
    }
}

#[async_trait]
impl RefTarget for Attribute {
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        let (id, external_id, slug) = entity_ref.to_owned().take_all_inner();
        Ok(get_attribute_id()
            .bind(
                client,
                &organization_id.into(),
                &id,
                &external_id.as_deref(),
                &slug.as_deref(),
            )
            .opt()
            .await?
            .map(|id| id.into()))
    }
}

/// Attribute, for creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAttribute {
    pub r#type: Ref<AttributeType>,
    pub title: I18nString,
    #[serde(default)]
    pub description: I18nString,
    pub slug: Option<Slug<Attribute>>,
    pub external_id: Option<ExternalId<Attribute>>,
}

impl SlugEntity for CreateAttribute {
    type RefTarget = Attribute;
    fn generate_slug(&self, prefix: &str) -> Option<Slug<Self::RefTarget>> {
        Some(Slug::new(slugify(&[prefix, &self.title.en])))
    }

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for CreateAttribute {
    type RefTarget = Attribute;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

/// Attribute, for update
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAttribute {
    pub r#type: Option<Ref<AttributeType>>,
    pub title: Option<I18nString>,
    pub description: Option<I18nString>,
    pub slug: Option<Slug<Attribute>>,
    pub external_id: Option<ExternalId<Attribute>>,
}

impl From<CreateAttribute> for UpdateAttribute {
    fn from(attr: CreateAttribute) -> Self {
        UpdateAttribute {
            r#type: Some(attr.r#type),
            title: Some(attr.title),
            description: Some(attr.description),
            slug: attr.slug,
            external_id: attr.external_id,
        }
    }
}

impl SlugEntity for UpdateAttribute {
    type RefTarget = Attribute;

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for UpdateAttribute {
    type RefTarget = Attribute;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}
