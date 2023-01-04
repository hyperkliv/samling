use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Query},
    RequestPartsExt,
};
use http::request::Parts;
use schemars::JsonSchema;
use time::Date;

use crate::{sorting::Sortable, Error, NestedStyle, Result};

#[derive(Debug, Copy, Clone, Default, serde::Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NestedStyleSortOrder {
    #[default]
    #[serde(rename = "number:asc", alias = "number")]
    Number,
    #[serde(rename = "name:asc", alias = "name")]
    Name,
    #[serde(rename = "delivery_period:asc", alias = "delivery_period")]
    DeliveryPeriodMin,
    #[serde(rename = "delivery_period:desc")]
    DeliveryPeriodMax,
}

impl Sortable for NestedStyleSortOrder {
    type Type = NestedStyle;
    fn sort(&self, mut styles: Vec<Self::Type>) -> Vec<Self::Type> {
        match self {
            Self::Number => {
                styles.sort_by_key(|style| style.number.clone());
                styles
            }
            Self::Name => {
                styles.sort_by_key(|style| style.name.clone());
                styles
            }
            Self::DeliveryPeriodMin => {
                styles.sort_by_cached_key(|style| {
                    style.colors.iter().fold(Date::MAX, |date1, color| {
                        let date2 = color.sizes.iter().fold(Date::MAX, |d1, size| {
                            let d2 = size.delivery_period.unwrap_or(Date::MAX);
                            d1.min(d2)
                        });
                        date1.min(date2)
                    })
                });
                styles
            }
            Self::DeliveryPeriodMax => {
                let mut sorted = Self::DeliveryPeriodMin.sort(styles);
                sorted.reverse();
                sorted
            }
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
struct SortableQuery {
    #[serde(default)]
    sort_by: NestedStyleSortOrder,
}

/// Extract organization_id from HTTP path
#[async_trait]
impl<S> FromRequestParts<S> for NestedStyleSortOrder
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        Ok(parts.extract::<Query<SortableQuery>>().await?.sort_by)
    }
}
