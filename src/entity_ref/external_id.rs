use std::{fmt::Debug, hash::Hash, marker::PhantomData, ops::Deref};

use async_trait::async_trait;
use cornucopia_async::GenericClient;

use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};

use super::{Id, Ref, RefTarget, RefType};
use crate::{organizations::Organization, Error, Result};

#[derive(Clone, Serialize, Deserialize, derive_more::Display)]
#[display("{_0}")]
#[serde(transparent)]
pub struct ExternalId<T: RefTarget>(String, PhantomData<T>);

impl<T: RefTarget> Debug for ExternalId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = Self::short_type_name();
        f.debug_tuple(&format!(r#"ExternalId({}:"{}")"#, type_name, self.0))
            .finish()
    }
}

impl<T: RefTarget> ExternalId<T> {
    pub fn new(value: String) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: RefTarget> RefType<T> for ExternalId<T> {
    type Inner = String;

    fn take(self) -> Self::Inner {
        self.0
    }

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn not_found_error(&self) -> Error {
        Error::ExternalIdNotFound(Self::short_type_name(), self.to_string())
    }

    fn already_exists_error(&self) -> Error {
        Error::ExternalIdAlreadyExists(Self::short_type_name(), self.to_string())
    }
}

impl<T: RefTarget> AsRef<str> for ExternalId<T> {
    fn as_ref(&self) -> &str {
        self.inner().as_str()
    }
}

impl<T: RefTarget> Deref for ExternalId<T> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl<T: RefTarget> Hash for ExternalId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: RefTarget> PartialEq for ExternalId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: RefTarget> Eq for ExternalId<T> {}

impl<T: RefTarget> PartialOrd for ExternalId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: RefTarget> Ord for ExternalId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        String::cmp(&self.0, &other.0)
    }
}

impl<T: RefTarget> From<String> for ExternalId<T> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<T: RefTarget> From<ExternalId<T>> for Ref<T> {
    fn from(val: ExternalId<T>) -> Self {
        Self::ExternalId(val)
    }
}

impl<T: RefTarget> From<ExternalId<T>> for String {
    fn from(external_id: ExternalId<T>) -> Self {
        external_id.0
    }
}

/// An entity that has an ExternalId field
#[async_trait]
pub trait ExternalIdEntity: core::fmt::Debug + Send + Sync {
    type RefTarget: RefTarget;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>>;
    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>);

    fn update_external_id_from_ref(&mut self, ref_: &Ref<Self::RefTarget>) {
        if let Some(new) = ref_.external_id() {
            self.set_external_id(new);
        }
    }

    async fn ensure_available_external_id(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<()> {
        if let Some(external_id) = self.external_id() {
            let potential_error = external_id.already_exists_error();
            let entity_ref = Ref::ExternalId(external_id);
            let res = Self::RefTarget::lookup_id(client, organization_id, &entity_ref).await?;
            match res {
                Some(_) => Err(potential_error),
                None => Ok(()),
            }
        } else {
            Ok(())
        }
    }
}

impl<T: RefTarget> JsonSchema for ExternalId<T> {
    fn schema_name() -> String {
        "ExternalId".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::ExternalIdEntity;
    use crate::{CreateStyle, ExternalId, Ref, Style};

    #[test]
    fn ensure_external_id_updated_from_ref() {
        let ref_ = Ref::ExternalId(ExternalId::<Style>::new("ERP:12345".into()));
        let mut style = CreateStyle::default();
        style.update_external_id_from_ref(&ref_);
        assert_eq!(style.external_id, Some(ExternalId::new("ERP:12345".into())));
    }
}
