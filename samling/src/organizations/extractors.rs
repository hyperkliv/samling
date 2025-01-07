use axum::{
    extract::{FromRequestParts, Path},
    RequestPartsExt,
};
use http::request::Parts;
use serde::Deserialize;

use crate::{entity_ref::Id, Error, Result};

use super::Organization;

#[derive(Debug, Clone, Copy, derive_more::Into, Deserialize)]
pub(crate) struct PathOrganizationId(pub(crate) Id<Organization>);

#[derive(Deserialize)]
struct OrganizationIdParam {
    organization_id: PathOrganizationId,
}

/// Extract organization_id from HTTP path
impl<S> FromRequestParts<S> for PathOrganizationId
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let path = parts.extract::<Path<OrganizationIdParam>>().await?;
        Ok(path.organization_id)
    }
}
