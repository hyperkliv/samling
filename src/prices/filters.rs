use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Query},
    RequestPartsExt,
};
use cornucopia_async::GenericClient;
use http::request::Parts;
use itertools::Itertools;
use serde::Deserialize;

use crate::{Collection, Error, Id, Organization, Ref, RefTarget, Result};

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct PriceListFilters {
    pub collections: Option<Vec<Ref<Collection>>>,
}

impl PriceListFilters {
    pub(crate) async fn collection_ids(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Option<Vec<Id<Collection>>>> {
        if let Some(refs) = self.collections.clone() {
            Ok(Some(
                Collection::lookup_ids(client, organization_id, &refs)
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

#[async_trait]
impl<S> FromRequestParts<S> for PriceListFilters
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let filters = parts.extract::<Query<PriceListFilters>>().await?.0;
        Ok(filters)
    }
}
