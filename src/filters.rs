use async_trait::async_trait;
use cornucopia_async::GenericClient;

use crate::{Id, Organization, Result};

#[async_trait]
pub trait Filters {
    type Resolved: ResolvedFilters;

    async fn resolve(
        self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Self::Resolved>;
}

pub trait ResolvedFilters {
    type Item;

    // Whether this item should be kept or filtered out
    fn keep(&self, item: &Self::Item) -> bool;
}
