mod any_id;
mod id;
pub(crate) use any_id::*;
use async_trait::async_trait;
use cornucopi_async::GenericClient;
use futures::future::try_join_all;
pub use id::*;
mod external_id;
pub use external_id::*;
use itertools::Itertools;
mod slug;
pub use self::slug::*;
mod ref_;
pub use ref_::*;
mod request;
pub use request::*;

use std::fmt::Display;

use serde::{de::DeserializeOwned, Serialize};

use crate::helpers;
use crate::organizations::Organization;
use crate::Error;
use crate::Result;

#[async_trait]
pub trait RefTarget: core::fmt::Debug + Clone + Send + Sync {
    /// Try to look up the entity's ID from the given reference
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &Ref<Self>,
    ) -> Result<Option<Id<Self>>>;

    /// Try to look up entity's IDs from the given references
    async fn lookup_ids(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_refs: &[Ref<Self>],
    ) -> Result<Vec<Option<Id<Self>>>> {
        let ret = try_join_all(
            entity_refs
                .iter()
                .map(|ref_| Self::lookup_id(client, organization_id, ref_)),
        )
        .await?;
        Ok(ret.into_iter().collect_vec())
    }

    /// Returns the looked up Id, or default to Id(0)
    ///
    /// Useful for example when you want a database query to return no row if the ref
    /// is not found.
    async fn lookup_id_or_default(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &Ref<Self>,
    ) -> Result<Id<Self>> {
        Ok(Self::lookup_id(client, organization_id, entity_ref)
            .await?
            .unwrap_or_else(|| Id::new(0)))
    }

    /// Look up entity's IDs from the given references, default to Id(0)
    ///
    /// Useful for example when you want a database query to return no rows on refs
    /// that are not found.
    async fn lookup_ids_with_default(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_refs: &[Ref<Self>],
    ) -> Result<Vec<Id<Self>>> {
        Ok(try_join_all(
            entity_refs
                .iter()
                .map(|ref_| Self::lookup_id_or_default(client, organization_id, ref_)),
        )
        .await?)
    }

    /// Return the entity's ID from the given reference, or return a not found error
    async fn get_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &Ref<Self>,
    ) -> Result<Id<Self>> {
        if let Some(id) = Self::lookup_id(client, organization_id, entity_ref).await? {
            Ok(id)
        } else {
            Err(entity_ref.not_found_error())
        }
    }

    /// Return if the given entity exists
    async fn exists(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &Ref<Self>,
    ) -> Result<bool> {
        Ok(Self::lookup_id(client, organization_id, entity_ref)
            .await?
            .is_some())
    }

    fn short_type_name() -> &'static str {
        helpers::short_type_name::<Self>()
    }
}

pub trait RefType<T: RefTarget>:
    Clone
    + core::fmt::Debug
    + Eq
    + PartialEq
    + Serialize
    + DeserializeOwned
    + Display
    + Send
    + Sync
    + From<Self::Inner>
{
    type Inner;
    fn take(self) -> Self::Inner;
    fn inner(&self) -> &Self::Inner;
    fn not_found_error(&self) -> Error;
    fn already_exists_error(&self) -> Error;

    fn short_type_name() -> &'static str {
        T::short_type_name()
    }
}
