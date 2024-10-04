use std::fmt::{Debug, Write as _};
use std::hash::Hash;
use std::{marker::PhantomData, ops::Deref};

use async_trait::async_trait;
use cornucopia_async::GenericClient;

use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Schema, SchemaObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{Id, Ref, RefTarget, RefType};
use crate::organizations::Organization;
use crate::{Error, Result};

#[derive(Clone, Serialize, Deserialize, derive_more::Display)]
#[display("{_0}")]
#[serde(transparent)]
pub struct Slug<T: RefTarget>(String, PhantomData<T>);

impl<T: RefTarget> Debug for Slug<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = Self::short_type_name();
        f.debug_tuple(&format!(r#"Slug({}:"{}")"#, type_name, self.0))
            .finish()
    }
}

impl<T: RefTarget> Slug<T> {
    pub fn new(value: String) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: RefTarget> RefType<T> for Slug<T> {
    type Inner = String;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn take(self) -> Self::Inner {
        self.0
    }

    fn not_found_error(&self) -> Error {
        Error::SlugNotFound(Self::short_type_name(), self.to_string())
    }

    fn already_exists_error(&self) -> Error {
        Error::SlugAlreadyExists(Self::short_type_name(), self.to_string())
    }
}

impl<T: RefTarget> AsRef<str> for Slug<T> {
    fn as_ref(&self) -> &str {
        self.inner().as_str()
    }
}

impl<T: RefTarget> Deref for Slug<T> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl<T: RefTarget> From<Slug<T>> for Ref<T> {
    fn from(val: Slug<T>) -> Self {
        Self::Slug(val)
    }
}

impl<T: RefTarget> Hash for Slug<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: RefTarget> PartialEq for Slug<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: RefTarget> Eq for Slug<T> {}

impl<T: RefTarget> PartialOrd for Slug<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: RefTarget> Ord for Slug<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        String::cmp(&self.0, &other.0)
    }
}

impl<T: RefTarget> From<String> for Slug<T> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<T: RefTarget> From<Slug<T>> for String {
    fn from(slug: Slug<T>) -> Self {
        slug.0
    }
}

/// An entity that has a Slug field
#[async_trait]
pub(crate) trait SlugEntity: core::fmt::Debug + Send + Sync {
    type RefTarget: RefTarget;

    fn generate_slug(&self, _prefix: &str) -> Option<Slug<Self::RefTarget>> {
        None
    }

    fn slug(&self) -> Option<Slug<Self::RefTarget>>;
    fn set_slug(&mut self, value: Slug<Self::RefTarget>);

    fn update_slug_from_ref(&mut self, ref_: &Ref<Self::RefTarget>) {
        if let Some(new) = ref_.slug() {
            self.set_slug(new);
        }
    }

    // /// Ensure that the given slug does not already exist, or generate a new one
    async fn check_or_generate_slug(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        prefix: &str,
    ) -> Result<Slug<Self::RefTarget>> {
        if let Some(slug) = self.slug() {
            if slug.is_empty() {
                Err(Error::EmptySlugDisallowed)
            } else {
                Self::_ensure_available_slug_impl(client, organization_id, slug.clone()).await?;
                dbg!(&slug);
                Ok(slug)
            }
        } else {
            for try_number in 0..i16::MAX {
                let res = self
                    ._check_or_generate_slug_impl(client, organization_id, prefix, try_number)
                    .await;
                if let Err(Error::SlugAlreadyExists(_, _)) = res {
                    continue;
                }
                return res;
            }
            let slug = Slug::<Self::RefTarget>::new(format!(
                "{}-{}",
                self.generate_slug(prefix)
                    .unwrap_or_else(|| Slug::new("".into())),
                i16::MAX
            ));
            Err(slug.already_exists_error())
        }
    }

    async fn _check_or_generate_slug_impl(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        prefix: &str,
        try_number: i16,
    ) -> Result<Slug<Self::RefTarget>> {
        match self.slug() {
            Some(slug) => {
                if slug.inner().is_empty() {
                    return Err(Error::EmptySlugDisallowed);
                }
                Self::_ensure_available_slug_impl(client, organization_id, slug.clone()).await?;
                Ok(slug)
            }
            None => {
                if let Some(mut slug) = self.generate_slug(prefix) {
                    if try_number > 0 {
                        let mut val = slug.take();
                        write!(val, "-{try_number}").unwrap();
                        slug = Slug::new(val);
                    }
                    Self::_ensure_available_slug_impl(client, organization_id, slug.clone())
                        .await?;
                    Ok(slug)
                } else {
                    Err(Error::EmptySlugDisallowed)
                }
            }
        }
    }

    /// Ensure that the given slug is not already taken
    async fn ensure_available_slug(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<()> {
        if let Some(slug) = self.slug() {
            Self::_ensure_available_slug_impl(client, organization_id, slug).await?;
        }
        Ok(())
    }

    /// Ensure that the given slug is not already taken
    async fn _ensure_available_slug_impl(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        slug: Slug<Self::RefTarget>,
    ) -> Result<()> {
        let entity_ref = super::Ref::Slug(slug);
        if Self::RefTarget::exists(client, organization_id, &entity_ref).await? {
            Err(entity_ref.slug().unwrap().already_exists_error())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SlugEntity;
    use crate::{CreateStyle, Ref, Slug, Style};

    #[test]
    fn ensure_slug_updated_from_ref() {
        let ref_ = Ref::Slug(Slug::<Style>::new("some-tshirt".into()));
        let mut style = CreateStyle::default();
        style.update_slug_from_ref(&ref_);
        assert_eq!(style.slug, Some(Slug::new("some-tshirt".into())));
    }
}

impl<T: RefTarget> JsonSchema for Slug<T> {
    fn schema_name() -> String {
        "Slug".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}
