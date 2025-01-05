use axum::{
    extract::{FromRequestParts, Query},
    RequestPartsExt,
};
use cornucopi_async::GenericClient;
use futures::future::try_join4;
use http::request::Parts;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    Attribute, Category, Error, Filters, Id, NestedStyle, Organization, PriceList, Ref, RefTarget,
    ResolvedFilters, Result, Style,
};

#[derive(Debug, Clone, Deserialize, Default, JsonSchema)]
#[serde(default)]
pub struct StyleFilters {
    pub refs: Option<Vec<Ref<Style>>>,
    pub pricelists: Option<Vec<Ref<PriceList>>>, // TODO: Not a filter! It's a data return modifier!
    pub categories: Option<Vec<Ref<Category>>>,
    pub attributes: Option<Vec<Ref<Attribute>>>,
    pub numbers: Option<Vec<String>>,
    pub service_item: Option<bool>,
    pub core: Option<bool>,
    pub new_styles: Option<bool>,
    pub new_colors: Option<bool>,
    pub country_of_origin: Option<Vec<String>>,
    pub status: Option<Vec<String>>,
}

impl Filters for StyleFilters {
    type Resolved = ResolvedStyleFilters;

    async fn resolve(
        self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<ResolvedStyleFilters> {
        let refs = self.refs.unwrap_or_default();
        let pricelist_refs = self.pricelists.clone().unwrap_or_default();
        let category_refs = self.categories.unwrap_or_default();
        let attribute_refs = self.attributes.unwrap_or_default();
        let (ids, pricelist_ids, category_ids, attribute_ids) = try_join4(
            Style::lookup_ids_with_default(client, organization_id, &refs),
            PriceList::lookup_ids_with_default(client, organization_id, &pricelist_refs),
            Category::lookup_ids_with_default(client, organization_id, &category_refs),
            Attribute::lookup_ids_with_default(client, organization_id, &attribute_refs),
        )
        .await?;
        let ids = if ids.is_empty() { None } else { Some(ids) };
        let pricelist_ids = if self.pricelists.is_some() {
            Some(pricelist_ids)
        } else {
            None
        };
        let category_ids = (!category_ids.is_empty()).then_some(category_ids);
        let attribute_ids = (!attribute_ids.is_empty()).then_some(attribute_ids);
        Ok(ResolvedStyleFilters {
            ids,
            pricelist_ids, // TODO: This is not used as a filter!
            category_ids,
            attribute_ids,
            numbers: self.numbers,
            service_item: self.service_item,
            core: self.core,
            new_styles: self.new_styles,
            new_colors: self.new_colors,
            country_of_origin: self.country_of_origin,
            status: self.status,
        })
    }
}

impl StyleFilters {
    pub(crate) fn from_ref(ref_: &crate::Ref<crate::Style>) -> Self {
        Self {
            refs: Some(vec![ref_.to_owned()]),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ResolvedStyleFilters {
    pub ids: Option<Vec<Id<Style>>>,
    pub pricelist_ids: Option<Vec<Id<PriceList>>>,
    pub category_ids: Option<Vec<Id<Category>>>,
    pub attribute_ids: Option<Vec<Id<Attribute>>>,
    pub numbers: Option<Vec<String>>,
    pub service_item: Option<bool>,
    pub core: Option<bool>,
    pub new_styles: Option<bool>,
    pub new_colors: Option<bool>,
    pub country_of_origin: Option<Vec<String>>,
    pub status: Option<Vec<String>>,
}

impl ResolvedFilters for ResolvedStyleFilters {
    type Item = NestedStyle;

    fn keep(&self, style: &Self::Item) -> bool {
        // TODO: Not a filter! A data modifier!
        // if let Some(pricelist_ids) = self.pricelist_ids {
        // }
        if let Some(ids) = &self.ids {
            if !ids.contains(&style.id) {
                return false;
            }
        }
        if let Some(category_ids) = &self.category_ids {
            if !style
                .categories
                .iter()
                .any(|cat| category_ids.contains(&cat.id))
            {
                return false;
            }
        }
        if let Some(numbers) = &self.numbers {
            if !numbers.contains(&style.number) {
                return false;
            }
        }
        if let Some(service_item) = self.service_item {
            if service_item && !style.service_item() {
                return false;
            }
        }
        if let Some(core) = self.core {
            if style.core != Some(core) {
                return false;
            }
        }
        if let Some(new_styles) = self.new_styles {
            if style.is_new != Some(new_styles) {
                return false;
            }
        }
        if let Some(new_colors) = self.new_colors {
            if style.has_new_color() != new_colors {
                return false;
            }
        }
        if let Some(country_of_origin) = &self.country_of_origin {
            if let Some(style_country) = &style.country_of_origin {
                if !country_of_origin.contains(style_country) {
                    return false;
                }
            } else {
                return false;
            }
        }
        if let Some(statuses) = &self.status {
            if !style.sizes().iter().any(|size| {
                if let Some(size_status) = &size.status {
                    statuses.contains(size_status)
                } else {
                    false
                }
            }) {
                return false;
            }
        }
        true
    }
}

impl ResolvedStyleFilters {
    // TODO: Prices shouldn't be part of filters - we're just
    //       removing nested data not the styles themselves!
    pub(crate) fn remove_unauthorized_prices(
        &self,
        mut item: NestedStyle,
        allowed_pricelists: &[Id<PriceList>],
    ) -> NestedStyle {
        item.prices = item
            .prices
            .drain(..)
            .filter(|price| allowed_pricelists.contains(&price.list.id))
            .collect();
        item
    }
}

impl<S> FromRequestParts<S> for StyleFilters
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let filters = parts.extract::<Query<StyleFilters>>().await?.0;
        Ok(filters)
    }
}
