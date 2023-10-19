use std::{cmp::Ordering, hash::Hash, str::FromStr};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{ExternalId, Id, RefTarget, RefType, Slug};
use crate::{Error, Result};

#[derive(Debug, Serialize, Deserialize, Clone, derive_more::Display, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Ref<T: RefTarget> {
    #[display(fmt = "{_0}")]
    Id(Id<T>),
    #[display(fmt = "external_id:{_0}")]
    ExternalId(ExternalId<T>),
    #[display(fmt = "slug:{_0}")]
    Slug(Slug<T>),
}

impl<T: RefTarget> Ref<T> {
    pub fn id(&self) -> Option<Id<T>> {
        match self {
            Self::Id(val) => Some(val.to_owned()),
            _ => None,
        }
    }

    pub fn slug(&self) -> Option<Slug<T>> {
        match self {
            Self::Slug(val) => Some(val.to_owned()),
            _ => None,
        }
    }

    pub fn external_id(&self) -> Option<ExternalId<T>> {
        match self {
            Self::ExternalId(val) => Some(val.to_owned()),
            _ => None,
        }
    }

    pub fn parse(value: String) -> Result<Self> {
        match value.split_once(':') {
            Some((ref_type_str, real_value)) => match ref_type_str {
                "id" => Ok(Self::Id(Id::from_str(real_value)?)),
                // Empty ref type str means that value looks like ":12345-Some-Item"
                "" | "external_id" => Ok(Self::ExternalId(ExternalId::new(real_value.into()))),
                "slug" => Ok(Self::Slug(Slug::new(real_value.into()))),
                _ => Err(Error::InvalidEntityRef(format!(
                    "Invalid ref type {ref_type_str}, valid choices are: id|slug|external_id"
                ))),
            },
            None => Self::parse(format!("id:{value}")),
        }
    }

    pub fn not_found_error(&self) -> Error {
        match self {
            Self::Id(id) => id.not_found_error(),
            Self::ExternalId(external_id) => external_id.not_found_error(),
            Self::Slug(slug) => slug.not_found_error(),
        }
    }

    pub fn take_all_inner(self) -> (Option<i32>, Option<String>, Option<String>) {
        match self {
            Self::Id(val) => (Some(val.take()), None, None),
            Self::ExternalId(val) => (None, Some(val.take()), None),
            Self::Slug(val) => (None, None, Some(val.take())),
        }
    }
}

impl<T: RefTarget> Hash for Ref<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Ref::Id(id) => id.hash(state),
            Ref::ExternalId(external_id) => external_id.hash(state),
            Ref::Slug(slug) => slug.hash(state),
        }
    }
}

impl<T: RefTarget> PartialEq for Ref<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Ref::Id(id1), Ref::Id(id2)) => id1.eq(id2),
            (Ref::ExternalId(external_id1), Ref::ExternalId(external_id2)) => {
                external_id1.eq(external_id2)
            }
            (Ref::Slug(slug1), Ref::Slug(slug2)) => slug1.eq(slug2),
            _ => false,
        }
    }
}

impl<T: RefTarget> Eq for Ref<T> {}

impl<T: RefTarget> PartialOrd for Ref<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: RefTarget> Ord for Ref<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Ref::Id(id1), Ref::Id(id2)) => id1.cmp(id2),
            (Ref::ExternalId(external_id1), Ref::ExternalId(external_id2)) => {
                external_id1.cmp(external_id2)
            }
            (Ref::Slug(slug1), Ref::Slug(slug2)) => slug1.cmp(slug2),
            _ => Ordering::Less,
        }
    }
}
