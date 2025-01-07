use samling_clorinde::client::GenericClient;

use crate::{Id, Organization, Result};

pub trait Filters {
    type Resolved: ResolvedFilters;

    #[allow(async_fn_in_trait)]
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
