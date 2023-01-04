use async_trait::async_trait;
use cornucopia_async::GenericClient;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    CollectionWithItems, Filters, Id, Organization, ResolvedFilters, ResolvedStyleFilters, Result,
    StyleFilters,
};

#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
pub struct CollectionFilters {
    #[serde(default)]
    pub styles: StyleFilters,
}

#[async_trait]
impl Filters for CollectionFilters {
    type Resolved = ResolvedCollectionFilters;

    async fn resolve(
        self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Self::Resolved> {
        Ok(Self::Resolved {
            styles: self.styles.resolve(client, organization_id).await?,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct ResolvedCollectionFilters {
    pub styles: ResolvedStyleFilters,
}

impl ResolvedFilters for ResolvedCollectionFilters {
    type Item = CollectionWithItems;

    fn keep(&self, collection: &Self::Item) -> bool {
        collection
            .items
            .iter()
            .any(|item| self.styles.keep(&item.style))
    }
}
