use std::{fmt::Debug, hash::Hash, marker::PhantomData, str::FromStr};

use postgres_types::{private::BytesMut, to_sql_checked, IsNull, ToSql, Type};
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};

use super::{Ref, RefTarget, RefType};
use crate::Error;

#[derive(Serialize, Deserialize, derive_more::Display)]
#[display(fmt = "{_0}")]
#[serde(transparent)]
pub struct Id<T: RefTarget>(i32, PhantomData<T>);

impl<T: RefTarget> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = Self::short_type_name();
        f.debug_tuple(&format!("Id({}:{})", type_name, self.0))
            .finish()
    }
}

impl<T: RefTarget> ToSql for Id<T> {
    fn to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        <i32 as ToSql>::to_sql(self.inner(), ty, w)
    }

    fn accepts(ty: &Type) -> bool {
        <i32 as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}

/// Manually implemented Copy and Clone because of the following reason:
/// "There is a small difference between the two: the derive strategy will
/// also place a Copy bound on type parameters, which isn’t always desired."
/// https://doc.rust-lang.org/stable/std/marker/trait.Copy.html#how-can-i-implement-copy
impl<T: RefTarget> Copy for Id<T> {}

impl<T: RefTarget> Clone for Id<T> {
    fn clone(&self) -> Id<T> {
        *self
    }
}

impl<T: RefTarget> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: RefTarget> RefType<T> for Id<T> {
    type Inner = i32;

    fn take(self) -> Self::Inner {
        self.0
    }

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn not_found_error(&self) -> Error {
        Error::IdNotFound(Self::short_type_name(), *self.inner())
    }

    fn already_exists_error(&self) -> Error {
        Error::IdAlreadyExists(Self::short_type_name(), *self.inner())
    }
}

impl<T: RefTarget> Id<T> {
    pub fn new(value: i32) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: RefTarget> From<Id<T>> for Ref<T> {
    fn from(val: Id<T>) -> Self {
        Self::Id(val)
    }
}

impl<T: RefTarget> FromStr for Id<T> {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let type_name = Self::short_type_name();
        let res = s
            .parse::<i32>()
            .map_err(|_| Error::InvalidEntityRef(format!("`{s}` is not a valid {type_name} ID")))?;
        Ok(Id::new(res))
    }
}

impl<T: RefTarget> From<i32> for Id<T> {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl<T: RefTarget> From<Id<T>> for i32 {
    fn from(id: Id<T>) -> Self {
        id.0
    }
}

impl<'a, T: RefTarget> From<&'a Id<T>> for &'a i32 {
    fn from(id: &'a Id<T>) -> Self {
        &id.0
    }
}

impl<'a, T: RefTarget> From<&'a Id<T>> for i32 {
    fn from(id: &'a Id<T>) -> Self {
        id.0
    }
}

impl<T: RefTarget> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: RefTarget> Eq for Id<T> {}

impl<T: RefTarget> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: RefTarget> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        i32::cmp(&self.0, &other.0)
    }
}

impl<T: RefTarget> JsonSchema for Id<T> {
    fn schema_name() -> String {
        "Id".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Number.into()),
            ..Default::default()
        }
        .into()
    }
}
