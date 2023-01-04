use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Query},
    RequestPartsExt,
};
use http::request::Parts;
use schemars::JsonSchema;

use crate::{sorting::Sortable, Error, Result, User};

#[derive(Debug, Copy, Clone, Default, serde::Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserSortOrder {
    #[default]
    #[serde(rename = "name:asc", alias = "name")]
    Name,
    #[serde(rename = "email:asc", alias = "email")]
    Email,
    #[serde(rename = "last_sign_in:asc", alias = "last_sign_in")]
    LastSignInOldest,
    #[serde(rename = "last_sign_in:desc")]
    LastSignInNewest,
}

impl Sortable for UserSortOrder {
    type Type = User;
    fn sort(&self, mut users: Vec<Self::Type>) -> Vec<Self::Type> {
        match self {
            Self::Name => {
                users.sort_by_key(|user| user.name.clone());
                users
            }
            Self::Email => {
                users.sort_by_key(|user| user.email.clone());
                users
            }
            Self::LastSignInOldest => {
                users.sort_by_key(|user| user.last_sign_in);
                users
            }
            Self::LastSignInNewest => {
                let mut sorted = Self::LastSignInOldest.sort(users);
                sorted.reverse();
                sorted
            }
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
struct SortableQuery {
    #[serde(default)]
    sort_by: UserSortOrder,
}

/// Extract organization_id from HTTP path
#[async_trait]
impl<S> FromRequestParts<S> for UserSortOrder
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        Ok(parts.extract::<Query<SortableQuery>>().await?.sort_by)
    }
}
