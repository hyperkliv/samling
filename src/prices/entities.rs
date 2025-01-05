use async_trait::async_trait;
use cornucopi_async::GenericClient;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::User,
    cornucopia::queries::{price::get_price_id, pricelist::get_pricelist_id},
    entity_ref::{ExternalId, ExternalIdEntity},
    helpers::slugify,
    organizations::Organization,
    EntityRefPathParam, Error, Id, Ref, RefTarget, Slug, SlugEntity, Style, StyleSummary,
};

/// PriceList
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PriceList {
    pub id: Id<Self>,
    pub slug: Slug<Self>,
    pub external_id: Option<ExternalId<Self>>,
    pub name: String,
    pub created_by: Option<Id<User>>,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
}

/// Price list summary
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PriceListSummary {
    pub id: Id<PriceList>,
    pub slug: Slug<PriceList>,
    pub external_id: Option<ExternalId<PriceList>>,
    pub name: String,
}

impl EntityRefPathParam for PriceList {
    fn parameter_name() -> &'static str {
        "price_list_ref"
    }
}

#[async_trait]
impl RefTarget for PriceList {
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        let (id, external_id, slug) = entity_ref.to_owned().take_all_inner();
        Ok(get_pricelist_id()
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

/// PriceList, for creation
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePriceList {
    pub name: String,
    pub slug: Option<Slug<PriceList>>,
    pub external_id: Option<ExternalId<PriceList>>,
}

impl ExternalIdEntity for CreatePriceList {
    type RefTarget = PriceList;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

impl SlugEntity for CreatePriceList {
    type RefTarget = PriceList;
    fn generate_slug(&self, prefix: &str) -> Option<Slug<Self::RefTarget>> {
        Some(Slug::new(slugify(&[prefix, &self.name])))
    }

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

/// PriceList, for update
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePriceList {
    pub name: Option<String>,
    pub slug: Option<Slug<PriceList>>,
    pub external_id: Option<ExternalId<PriceList>>,
}

impl From<CreatePriceList> for UpdatePriceList {
    fn from(pricelist: CreatePriceList) -> Self {
        UpdatePriceList {
            name: Some(pricelist.name),
            slug: pricelist.slug,
            external_id: pricelist.external_id,
        }
    }
}

impl SlugEntity for UpdatePriceList {
    type RefTarget = PriceList;
    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for UpdatePriceList {
    type RefTarget = PriceList;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

/// Price set (belonging to the same list and style, but with different start/end dates)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Price {
    pub id: Id<Self>,
    pub r#type: PriceType,
    pub uom: Option<String>,
    pub currency: String,
    #[serde(serialize_with = "rust_decimal::serde::float::serialize")]
    pub amount: Decimal,
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date")]
    pub start: time::Date,
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date")]
    pub end: time::Date,
    pub external_id: Option<ExternalId<Self>>,
    pub created_by: Option<Id<User>>,
    pub style: StyleSummary,
    pub list: PriceListSummary,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
}

/// Nested price set, for inclusion in a NestedStyle
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NestedPrice {
    pub id: Id<Price>,
    #[serde(serialize_with = "rust_decimal::serde::float::serialize")]
    pub amount: Decimal,
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date")]
    pub start: time::Date,
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date")]
    pub end: time::Date,
    pub r#type: PriceType,
    pub uom: Option<String>,
    pub currency: String,
    pub list: PriceListSummary,
}
impl NestedPrice {
    pub(crate) fn is_unit_price(&self) -> bool {
        self.r#type == PriceType::Unit
    }
    pub(crate) fn is_retail_price(&self) -> bool {
        self.r#type == PriceType::Retail
    }
}

impl EntityRefPathParam for Price {
    fn parameter_name() -> &'static str {
        "price_ref"
    }
}

#[async_trait]
impl RefTarget for Price {
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        let (id, external_id, slug) = entity_ref.to_owned().take_all_inner();
        if slug.is_some() {
            Err(Error::SlugReferenceUnsupported(Self::short_type_name()))
        } else {
            Ok(get_price_id()
                .bind(
                    client,
                    &organization_id.into(),
                    &id,
                    &external_id.as_deref(),
                )
                .opt()
                .await?
                .map(|id| id.into()))
        }
    }
}

/// Price, for creation
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePrice {
    pub r#type: PriceType,
    pub uom: Option<String>,
    pub currency: String,
    pub amount: Decimal,
    pub start: time::Date,
    pub end: time::Date,
    pub list: Ref<PriceList>,
    pub style: Ref<Style>,
    pub external_id: Option<ExternalId<Price>>,
}

impl ExternalIdEntity for CreatePrice {
    type RefTarget = Price;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

/// Price set, for update
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePrice {
    pub r#type: Option<PriceType>,
    pub uom: Option<String>,
    pub currency: Option<String>,
    pub amount: Option<Decimal>,
    pub start: Option<time::Date>,
    pub end: Option<time::Date>,
    pub style: Option<Ref<Style>>,
    pub list: Option<Ref<PriceList>>,
    pub external_id: Option<ExternalId<Price>>,
}

impl From<CreatePrice> for UpdatePrice {
    fn from(price: CreatePrice) -> Self {
        UpdatePrice {
            r#type: Some(price.r#type),
            uom: price.uom,
            currency: Some(price.currency),
            amount: Some(price.amount),
            start: Some(price.start),
            end: Some(price.end),
            style: Some(price.style),
            list: Some(price.list),
            external_id: price.external_id,
        }
    }
}

impl ExternalIdEntity for UpdatePrice {
    type RefTarget = Price;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, JsonSchema,
)]
pub enum PriceType {
    Unit,
    Retail,
}
