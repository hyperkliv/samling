use std::collections::HashSet;

use num_enum::TryFromPrimitive;
use postgres_types::ToSql;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::IntoEnumIterator;

/// A permission that can be given to a role
#[derive(
    Debug,
    Copy,
    Clone,
    Deserialize,
    Serialize,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::EnumIter,
    JsonSchema,
)]
pub enum Permission {
    SignIn,
    ListOrganizations,
    GetOrganization,
    UpdateOrganization,
    DeleteOrganization,
    CreateUser,
    ListUsers,
    GetUser,
    UpdateUser,
    DeleteUser,
    CreateCollection,
    ListCollections,
    GetCollection,
    UpdateCollection,
    DeleteCollection,
    ListStyles,
    ListCollectionItems,
    GetCollectionSummary,
    CreateStyle,
    GetStyle,
    UpdateStyle,
    DeleteStyle,
    ListColors,
    CreateColor,
    GetColor,
    UpdateColor,
    DeleteColor,
    ListSizes,
    GetSize,
    UpdateSize,
    DeleteSize,
    CreateSize,
    ListCategories,
    CreateCategory,
    GetCategory,
    UpdateCategory,
    DeleteCategory,
    ListImages,
    GetImage,
    CreateImage,
    UpdateImage,
    DeleteImage,
    ListPriceLists,
    GetPriceList,
    CreatePriceList,
    UpdatePriceList,
    DeletePriceList,
    ListPrices,
    GetPrice,
    CreatePrice,
    UpdatePrice,
    DeletePrice,
    GetCollectionItem,
    ExportStyles,
    CreateGroup,
    UpdateGroup,
    DeleteGroup,
    GetGroup,
    ListGroups,
    ListFilterChoices,
}

use Permission::*;

/// A role that can be assigned to a user
///
/// The u8 representation is meant for database storage/retrieval
#[derive(
    Debug,
    Copy,
    Clone,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    TryFromPrimitive,
    strum::EnumString,
    strum::Display,
    ToSql,
    EnumIter,
    JsonSchema,
)]
#[repr(u8)]
pub enum Role {
    Active = 0,
    Viewer = 1,
    Editor = 2,
    Administrator = 3,
}

impl From<Role> for i32 {
    fn from(role: Role) -> Self {
        role as i32
    }
}

impl From<&Role> for i32 {
    fn from(role: &Role) -> Self {
        *role as i32
    }
}

use Role::*;

use crate::{Error, Result};

impl Role {
    #[inline]
    pub const fn get_permissions(&self) -> &'static [Permission] {
        match self {
            Active => Self::get_active_permissions(),
            Viewer => Self::get_viewer_permissions(),
            Administrator => Self::get_administrator_permissions(),
            Editor => Self::get_editor_permissions(),
        }
    }

    pub fn all() -> Vec<Self> {
        Self::iter().collect::<Vec<_>>()
    }

    const fn get_active_permissions() -> &'static [Permission] {
        &[SignIn]
    }

    const fn get_viewer_permissions() -> &'static [Permission] {
        &[
            ExportStyles,
            GetCollection,
            GetCollectionItem,
            ListCollectionItems,
            GetCollectionSummary,
            ListCollections,
            ListPriceLists,
        ]
    }

    const fn get_editor_permissions() -> &'static [Permission] {
        &[
            CreateCollection,
            CreateGroup,
            DeleteGroup,
            ExportStyles,
            GetCategory,
            GetCollection,
            GetCollectionItem,
            GetCollectionSummary,
            GetColor,
            GetGroup,
            GetSize,
            GetStyle,
            ListCollectionItems,
            ListCollections,
            ListGroups,
            ListPriceLists,
            UpdateGroup,
        ]
    }

    const fn get_administrator_permissions() -> &'static [Permission] {
        &[
            CreateCategory,
            CreateCollection,
            CreateColor,
            CreateGroup,
            CreateImage,
            CreatePrice,
            CreatePriceList,
            CreateSize,
            CreateStyle,
            CreateUser,
            DeleteCategory,
            DeleteCollection,
            DeleteColor,
            DeleteGroup,
            DeleteImage,
            DeleteOrganization,
            DeletePrice,
            DeletePriceList,
            DeleteSize,
            DeleteStyle,
            DeleteUser,
            ExportStyles,
            GetCategory,
            GetCollection,
            GetCollectionItem,
            GetCollectionSummary,
            GetColor,
            GetGroup,
            GetImage,
            GetOrganization,
            GetPrice,
            GetPriceList,
            GetSize,
            GetStyle,
            GetUser,
            ListCategories,
            ListCollectionItems,
            ListCollections,
            ListColors,
            ListFilterChoices,
            ListGroups,
            ListImages,
            ListOrganizations,
            ListPriceLists,
            ListPrices,
            ListSizes,
            ListStyles,
            ListUsers,
            UpdateCategory,
            UpdateCollection,
            UpdateColor,
            UpdateGroup,
            UpdateImage,
            UpdateOrganization,
            UpdatePrice,
            UpdatePriceList,
            UpdateSize,
            UpdateStyle,
            UpdateUser,
        ]
    }

    pub(crate) fn from_db_id(id: i32) -> Result<Role> {
        let parsed_id: u8 = id
            .try_into()
            .map_err(|_| Error::InvalidDbRoleId(id).traced())?;
        if let Ok(role) = Role::try_from(parsed_id) {
            Ok(role)
        } else {
            Err(Error::InvalidDbRoleId(id).traced())
        }
    }

    pub(crate) fn from_db_ids(role_ids: &[i32]) -> Vec<Role> {
        let mut role_ids: Vec<Role> = role_ids
            .iter()
            .filter_map(|role_id| Role::from_db_id(*role_id).ok())
            .collect();
        role_ids.sort_unstable();
        role_ids.dedup();
        role_ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_enum::TryFromPrimitiveError;

    #[test]
    fn create_role_from_u8() {
        assert_eq!(Role::Active, 0_u8.try_into().unwrap());
        assert_eq!(Role::Viewer, 1_u8.try_into().unwrap());
        assert_eq!(Role::Editor, 2_u8.try_into().unwrap());
        assert_eq!(Role::Administrator, 3_u8.try_into().unwrap());
    }

    #[test]
    fn create_role_from_non_existing_u8() {
        assert_eq!(
            Role::try_from(100_u8).unwrap_err(),
            TryFromPrimitiveError { number: 100_u8 }
        );
    }

    #[test]
    fn from_db_ids_ignores_invalid_id() {
        assert_eq!(Role::from_db_ids(&[3, 1337]), vec![Role::Administrator]);
    }

    #[test]
    fn from_db_ids_deduplicates() {
        assert_eq!(Role::from_db_ids(&[0, 0, 0]), vec![Role::Active]);
    }

    #[test]
    fn from_db_ids_sorts_by_id() {
        assert_eq!(
            Role::from_db_ids(&[3, 2, 0]),
            vec![Role::Active, Role::Editor, Role::Administrator]
        );
    }
}

impl Permission {
    pub(crate) fn from_roles(roles: &[Role]) -> Vec<Permission> {
        let mut set = HashSet::new();
        for role in roles {
            for perm in role.get_permissions() {
                set.insert(*perm);
            }
        }
        let mut vec = Vec::from_iter(set);
        vec.sort_unstable();
        vec
    }
}
