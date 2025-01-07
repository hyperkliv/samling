use schemars::JsonSchema;
use serde::Serialize;

use crate::{I18nString, ImageSummary};

#[derive(Debug, Serialize, Clone, JsonSchema)]
pub struct ItemFilterChoices {
    pub status: Vec<StringFilterChoice>,
    pub category: Vec<EntityFilterChoice>,
    pub style: Vec<EntityFilterChoice>,
    pub attribute: Vec<EntityFilterChoice>,
}

#[derive(Debug, Serialize, Clone, JsonSchema, derive_more::From)]
pub struct StringFilterChoice(String);

#[derive(Debug, Serialize, Clone, JsonSchema)]
pub struct EntityFilterChoice {
    pub id: i32,
    pub title: I18nString,
    pub subtitle: Option<I18nString>,
    pub image: Option<ImageSummary>,
}
