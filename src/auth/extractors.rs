use async_trait::async_trait;
use axum::extract::{FromRequestParts, Path};
use axum::RequestPartsExt;
use http::request::Parts;
use serde::Deserialize;

use super::User;
use crate::{entity_ref::Id, Error, Result};

#[derive(Debug, Clone, Copy, derive_more::Into, Deserialize)]
pub(crate) struct PathUserId(pub(crate) Id<User>);

#[derive(Deserialize)]
struct UserIdPathParam {
    user_id: PathUserId,
}

/// Extract user_id from HTTP path
#[async_trait]
impl<S> FromRequestParts<S> for PathUserId
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let path = parts.extract::<Path<UserIdPathParam>>().await?;
        Ok(path.user_id)
    }
}
