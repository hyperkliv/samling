use axum::{
    extract::{FromRequestParts, Query},
    RequestPartsExt,
};
use cornucopi_async::GenericClient;
use http::request::Parts;
use itertools::Itertools;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Error, Group, Id, Organization, Ref, RefTarget, Result, Role};

#[derive(Debug, Clone, Deserialize, JsonSchema, Default)]
#[serde(default)]
pub struct UserFilters {
    pub roles: Option<Vec<Role>>,
    pub groups: Option<Vec<Ref<Group>>>,
}

impl UserFilters {
    pub(crate) async fn group_ids(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Option<Vec<Id<Group>>>> {
        if let Some(refs) = self.groups.clone() {
            Ok(Some(
                Group::lookup_ids(client, organization_id, &refs)
                    .await?
                    .into_iter()
                    .flatten()
                    .collect_vec(),
            ))
        } else {
            Ok(None)
        }
    }
}

impl<S> FromRequestParts<S> for UserFilters
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let filters = parts.extract::<Query<UserFilters>>().await?.0;
        Ok(filters)
    }
}
