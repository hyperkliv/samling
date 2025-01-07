use deadpool_postgres::Object;
use futures::future::try_join_all;
use itertools::Itertools;
use password_hash::PasswordHashString;
use samling_clorinde::client::{GenericClient, Params};
use serde::Deserialize;

use super::{
    filters::UserFilters, hashing::Hasher, sorting::UserSortOrder, CreateUser, UpdateUser, User,
};
use crate::{
    entity_ref::{Id, RefType},
    organizations::Organization,
    sorting::Sortable,
    Collection, CreateGroup, Error, ExternalIdEntity, Group, GroupSummary, OrganizationSummary,
    PriceList, Ref, RefTarget, Result, Role, SlugEntity, UpdateGroup, UserOrganization,
};
use samling_clorinde::queries::{
    group::{
        delete_group, insert_group, replace_group_collections, replace_group_pricelists,
        replace_group_users, select_group_summaries, select_groups, update_group,
        DeleteGroupParams, GroupRow, GroupRowBorrowed, GroupSummaryRow, GroupSummaryRowBorrowed,
        InsertGroupParams, ReplaceGroupCollectionsParams, ReplaceGroupPricelistsParams,
        ReplaceGroupUsersParams, SelectGroupSummariesParams, SelectGroupsParams, UpdateGroupParams,
    },
    user::{
        delete_user, insert_user, replace_user_groups, select_users, update_last_sign_in,
        update_user, upsert_user_organization, InsertUserParams, ReplaceUserGroupsParams,
        SelectUsersParams, UpdateUserParams, UpsertUserOrganizationParams, UserRow,
        UserRowBorrowed,
    },
};

#[derive(Clone)]
pub struct AuthRepo;

impl AuthRepo {
    pub async fn fetch_org_users(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        filters: UserFilters,
        sorter: UserSortOrder,
    ) -> Result<Vec<User>> {
        let roles = Some(
            filters
                .roles
                .clone()
                .unwrap_or_default()
                .iter()
                .map(i32::from)
                .collect_vec(),
        );
        let groups = filters
            .group_ids(client, organization_id)
            .await?
            .map(|vec| vec.into_iter().map(|id| id.into()).collect_vec());
        let users = select_users()
            .params(
                client,
                &SelectUsersParams {
                    organization_id: Some(organization_id.into()),
                    id: None,
                    email: None::<String>,
                    roles,
                    groups,
                },
            )
            .map(handle_user_row_borrowed)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()?;
        Ok(sorter.sort(users))
    }

    pub async fn find_user(
        &self,
        client: &impl GenericClient,
        id: Id<User>,
    ) -> Result<Option<User>> {
        if let Some(res) = select_users()
            .params(
                client,
                &SelectUsersParams {
                    organization_id: None,
                    id: Some(id.into()),
                    email: None::<String>,
                    roles: None::<Vec<_>>,
                    groups: None::<Vec<_>>,
                },
            )
            .map(handle_user_row_borrowed)
            .opt()
            .await?
        {
            Ok(Some(res?))
        } else {
            Ok(None)
        }
    }

    // Get user by ID
    pub async fn get_user(&self, client: &impl GenericClient, id: Id<User>) -> Result<User> {
        if let Some(user) = self.find_user(client, id).await? {
            Ok(user)
        } else {
            Err(id.not_found_error())
        }
    }

    // Get user by ID (which must be) associated to the given organization
    pub async fn get_org_user(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        id: Id<User>,
    ) -> Result<User> {
        if let Some(user) = select_users()
            .params(
                client,
                &SelectUsersParams {
                    organization_id: Some(organization_id.into()),
                    id: Some(id.into()),
                    email: None::<String>,
                    roles: None::<Vec<_>>,
                    groups: None::<Vec<_>>,
                },
            )
            .map(handle_user_row_borrowed)
            .opt()
            .await?
        {
            Ok(user?)
        } else {
            Err(id.not_found_error())
        }
    }

    pub async fn find_user_by_email(
        &self,
        client: &impl GenericClient,
        email: &str,
    ) -> Result<Option<User>> {
        if let Some(res) = select_users()
            .params(
                client,
                &SelectUsersParams {
                    organization_id: None,
                    id: None,
                    email: Some(email),
                    roles: None::<Vec<_>>,
                    groups: None::<Vec<_>>,
                },
            )
            .map(handle_user_row_borrowed)
            .opt()
            .await?
        {
            Ok(Some(res?))
        } else {
            Ok(None)
        }
    }

    pub async fn insert_user(
        &self,
        client: &impl GenericClient,
        user: CreateUser,
        password_hash: Option<PasswordHashString>,
    ) -> Result<User> {
        let existing = self.find_user_by_email(client, &user.email).await?;
        if existing.is_some() {
            return Err(Error::UserEmailAlreadyExists(user.email));
        }
        let inserted_id: Id<User> = insert_user()
            .params(
                client,
                &InsertUserParams {
                    name: user.name,
                    email: user.email,
                    password_hash: password_hash.as_ref().map(|ph| ph.as_ref()),
                    profile_image: user.profile_image.map(|url| url.to_string()),
                },
            )
            .one()
            .await?
            .into();
        self.get_user(client, inserted_id).await
    }

    pub async fn update_user(
        &self,
        client: &impl GenericClient,
        id: Id<User>,
        user: UpdateUser,
        password_hash: Option<PasswordHashString>,
    ) -> Result<User> {
        if let Some(existing) = self.find_user(client, id).await? {
            if let Some(email) = &user.email {
                if let Some(existing_by_email) = self.find_user_by_email(client, email).await? {
                    if existing_by_email.id != existing.id {
                        return Err(Error::UserEmailAlreadyExists(email.to_owned()));
                    }
                }
            }
            let num_deleted = update_user()
                .params(
                    client,
                    &UpdateUserParams {
                        name: user.name.as_deref(),
                        email: user.email.as_deref(),
                        password_hash: password_hash.as_ref().map(|ph| ph.as_ref()),
                        profile_image: user.profile_image.map(|url| url.to_string()),
                        id: id.into(),
                    },
                )
                .await?;
            debug_assert_eq!(num_deleted, 1);
            self.get_user(client, id).await
        } else {
            Err(id.not_found_error())
        }
    }

    pub async fn update_org_user(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        id: Id<User>,
        user_data: UpdateUser,
        password_hash: Option<PasswordHashString>,
    ) -> Result<User> {
        let tx = client.transaction().await?;
        let user = self
            .update_user(&tx, id, user_data.clone(), password_hash)
            .await?;
        self.associate_organization_and_roles(
            &tx,
            user.id,
            organization_id,
            user_data.roles.as_deref(),
        )
        .await?;
        if let Some(groups) = user_data.groups {
            self.associate_groups(&tx, user.id, organization_id, &groups)
                .await?;
        }
        tx.commit().await?;
        self.get_user(client, user.id).await
    }

    pub async fn upsert_user(
        &self,
        client: &mut Object,
        user: CreateUser,
        password_hash: Option<PasswordHashString>,
    ) -> Result<(bool, User)> {
        match self.find_user_by_email(client, &user.email).await? {
            Some(existing) => Ok((
                false,
                self.update_user(client, existing.id, user.to_owned().into(), password_hash)
                    .await?,
            )),
            None => Ok((
                true,
                self.insert_user(client, user.to_owned(), password_hash)
                    .await?,
            )),
        }
    }

    pub async fn insert_org_user(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        user_data: CreateUser,
        password_hash: Option<PasswordHashString>,
    ) -> Result<User> {
        let tx = client.transaction().await?;
        let user = self
            .insert_user(&tx, user_data.clone(), password_hash)
            .await?;
        self.associate_organization_and_roles(
            &tx,
            user.id,
            organization_id,
            user_data.roles.as_deref(),
        )
        .await?;
        if let Some(groups) = user_data.groups {
            self.associate_groups(&tx, user.id, organization_id, &groups)
                .await?;
        }
        tx.commit().await?;
        self.get_user(client, user.id).await
    }

    pub async fn delete_user(&self, client: &impl GenericClient, id: Id<User>) -> Result<()> {
        let num_deleted = delete_user().bind(client, &id.into()).await?;
        debug_assert_eq!(num_deleted, 1);
        Ok(())
    }

    pub async fn associate_organization_and_roles(
        &self,
        client: &impl GenericClient,
        user_id: Id<User>,
        organization_id: Id<Organization>,
        roles: Option<&[super::rbac::Role]>,
    ) -> Result<()> {
        let role_ids = roles.map(|r| r.iter().map(|r| r.into()).collect_vec());
        upsert_user_organization()
            .params(
                client,
                &UpsertUserOrganizationParams {
                    user_id: user_id.into(),
                    organization_id: organization_id.into(),
                    role_ids,
                },
            )
            .await?;
        Ok(())
    }

    pub async fn associate_groups(
        &self,
        client: &impl GenericClient,
        user_id: Id<User>,
        organization_id: Id<Organization>,
        group_refs: &[Ref<Group>],
    ) -> Result<()> {
        let group_ids: Vec<i32> = try_join_all(
            group_refs
                .iter()
                .map(|ref_| async { Group::get_id(client, organization_id, ref_).await }),
        )
        .await?
        .iter()
        .map(|v| v.into())
        .dedup()
        .collect();

        replace_user_groups()
            .params(
                client,
                &ReplaceUserGroupsParams {
                    user_id: user_id.into(),
                    group_ids,
                },
            )
            .all()
            .await?;

        Ok(())
    }

    pub async fn update_last_sign_in(
        &self,
        client: impl GenericClient,
        user_id: Id<User>,
    ) -> Result<()> {
        update_last_sign_in().bind(&client, &user_id.into()).await?;
        Ok(())
    }

    pub async fn authenticate(
        &self,
        client: &impl GenericClient,
        hasher: Hasher,
        user_login: &super::UserLogin,
    ) -> Result<User> {
        let err = Err(Error::InvalidUserCredentials);

        if let Some(user_row) = select_users()
            .params(
                client,
                &SelectUsersParams {
                    organization_id: None,
                    id: None,
                    email: Some(user_login.email.as_str()),
                    roles: None::<Vec<_>>,
                    groups: None::<Vec<_>>,
                },
            )
            .opt()
            .await?
        {
            if let Some(ph) = &user_row.password_hash {
                if hasher.verify(&user_login.password, ph).is_ok() {
                    let user = handle_user_row(user_row)?;
                    update_last_sign_in().bind(client, &user.id.into()).await?;
                    Ok(user)
                } else {
                    err
                }
            } else {
                err
            }
        } else {
            err
        }
    }

    pub async fn list_groups(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Vec<Group>> {
        select_groups()
            // TODO: Filtering
            .params(
                client,
                &SelectGroupsParams {
                    organization_id: organization_id.into(),
                    id: None,
                    external_id: None::<&str>,
                    slug: None::<&str>,
                },
            )
            .map(handle_group_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()
    }

    pub async fn list_group_summaries(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Vec<GroupSummary>> {
        select_group_summaries()
            // TODO: Filtering
            .params(
                client,
                &SelectGroupSummariesParams {
                    organization_id: organization_id.into(),
                    id: None,
                    external_id: None::<&str>,
                    slug: None::<&str>,
                },
            )
            .map(handle_summary_group_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()
    }

    pub async fn find_group(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Group>,
    ) -> Result<Option<Group>> {
        let (id, external_id, slug) = ref_.to_owned().take_all_inner();
        if let Some(res) = select_groups()
            .params(
                client,
                &SelectGroupsParams {
                    organization_id: organization_id.into(),
                    id,
                    external_id,
                    slug,
                },
            )
            .map(handle_group_row)
            .opt()
            .await?
        {
            Ok(Some(res?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_group(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Group>,
    ) -> Result<Group> {
        if let Some(group) = self.find_group(client, organization_id, ref_).await? {
            Ok(group)
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub(crate) async fn insert_group(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        group: CreateGroup,
        created_by: Id<User>,
    ) -> Result<Group> {
        let tx = client.transaction().await?;
        let slug = group
            .check_or_generate_slug(&tx, organization_id, "")
            .await?;
        group
            .ensure_available_external_id(&tx, organization_id)
            .await?;
        let inserted_id: Id<Group> = insert_group()
            .params(
                &tx,
                &InsertGroupParams {
                    organization_id: organization_id.into(),
                    slug: slug.as_ref(),
                    external_id: group.external_id.as_deref(),
                    name: &group.name,
                    description: &group.description,
                    created_by: created_by.into(),
                },
            )
            .one()
            .await?
            .into();

        self.associate_users(&tx, organization_id, &inserted_id, &group.users)
            .await?;
        self.associate_collections(&tx, organization_id, &inserted_id, &group.collections)
            .await?;
        self.associate_price_lists(&tx, organization_id, &inserted_id, &group.price_lists)
            .await?;
        let group = self
            .get_group(&tx, organization_id, &inserted_id.into())
            .await?;
        tx.commit().await?;
        Ok(group)
    }

    pub(crate) async fn update_group(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        ref_: &Ref<Group>,
        group: UpdateGroup,
    ) -> Result<Group> {
        let tx = client.transaction().await?;
        if let Some(existing) = self.find_group(&tx, organization_id, ref_).await? {
            if group.slug.is_some() && Some(existing.slug) != group.slug {
                group.ensure_available_slug(&tx, organization_id).await?;
            }
            if group.external_id.is_some() && existing.external_id != group.external_id {
                group
                    .ensure_available_external_id(&tx, organization_id)
                    .await?;
            }
            let num_updated = update_group()
                .params(
                    &tx,
                    &UpdateGroupParams {
                        slug: group.slug.as_deref(),
                        external_id: group.external_id.as_deref(),
                        name: group.name.as_ref(),
                        description: group.description.as_ref(),
                        id: existing.id.into(),
                    },
                )
                .await?;
            debug_assert_eq!(num_updated, 1);

            if let Some(users) = group.users {
                self.associate_users(&tx, organization_id, &existing.id, &users)
                    .await?;
            }
            if let Some(collections) = group.collections {
                self.associate_collections(&tx, organization_id, &existing.id, &collections)
                    .await?;
            }
            if let Some(price_lists) = group.price_lists {
                self.associate_price_lists(&tx, organization_id, &existing.id, &price_lists)
                    .await?;
            }
            let group = self
                .get_group(&tx, organization_id, &existing.id.into())
                .await?;
            tx.commit().await?;
            Ok(group)
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub async fn upsert_group(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        group_ref: &Ref<Group>,
        mut group: CreateGroup,
    ) -> Result<(bool, Group)> {
        group.update_slug_from_ref(group_ref);
        group.update_external_id_from_ref(group_ref);
        if Group::lookup_id(client, organization_id, group_ref)
            .await?
            .is_some()
        {
            Ok((
                false,
                self.update_group(client, organization_id, group_ref, group.into())
                    .await?,
            ))
        } else {
            Ok((
                true,
                self.insert_group(client, organization_id, group, created_by)
                    .await?,
            ))
        }
    }
    pub async fn delete_group(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Group>,
    ) -> Result<()> {
        let id = Group::get_id(client, organization_id, ref_).await?;
        let num_deleted = delete_group()
            .params(
                client,
                &DeleteGroupParams {
                    organization_id: organization_id.into(),
                    id: id.into(),
                },
            )
            .await?;
        debug_assert_eq!(num_deleted, 1);
        Ok(())
    }

    pub(crate) async fn associate_users(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        group_id: &Id<Group>,
        users: &[Ref<User>],
    ) -> Result<()> {
        let user_ids: Vec<i32> = try_join_all(
            users
                .iter()
                .map(|user_ref| User::lookup_id(client, organization_id, user_ref)),
        )
        .await?
        .into_iter()
        .zip(users)
        .filter_map(|(id, user_ref)| {
            if id.is_none() {
                tracing::info!("{user_ref} doesn't exist in database");
                None
            } else {
                id
            }
        })
        .map(|id| id.into())
        .collect();

        replace_group_users()
            .params(
                client,
                &ReplaceGroupUsersParams {
                    group_id: group_id.into(),
                    user_ids,
                },
            )
            .all()
            .await?;
        Ok(())
    }

    pub(crate) async fn associate_collections(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        group_id: &Id<Group>,
        collections: &[Ref<Collection>],
    ) -> Result<()> {
        let collection_ids: Vec<i32> =
            try_join_all(collections.iter().map(|collection_ref| {
                Collection::lookup_id(client, organization_id, collection_ref)
            }))
            .await?
            .into_iter()
            .zip(collections)
            .filter_map(|(id, collection_ref)| {
                if id.is_none() {
                    tracing::info!("{collection_ref} doesn't exist in database");
                    None
                } else {
                    id
                }
            })
            .map(|id| id.into())
            .collect();

        replace_group_collections()
            .params(
                client,
                &ReplaceGroupCollectionsParams {
                    group_id: group_id.into(),
                    collection_ids,
                },
            )
            .all()
            .await?;
        Ok(())
    }

    pub(crate) async fn associate_price_lists(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        group_id: &Id<Group>,
        price_lists: &[Ref<PriceList>],
    ) -> Result<()> {
        let pricelist_ids: Vec<i32> =
            try_join_all(price_lists.iter().map(|price_list_ref| {
                PriceList::lookup_id(client, organization_id, price_list_ref)
            }))
            .await?
            .into_iter()
            .zip(price_lists)
            .filter_map(|(id, price_list_ref)| {
                if id.is_none() {
                    tracing::info!("{price_list_ref} doesn't exist in database");
                    None
                } else {
                    id
                }
            })
            .map(|id| id.into())
            .collect();

        replace_group_pricelists()
            .params(
                client,
                &ReplaceGroupPricelistsParams {
                    group_id: group_id.into(),
                    pricelist_ids,
                },
            )
            .all()
            .await?;
        Ok(())
    }
}

fn handle_user_row_borrowed(row: UserRowBorrowed) -> Result<User> {
    let row: UserRow = row.into();
    handle_user_row(row)
}

fn handle_user_row(row: UserRow) -> Result<User> {
    let user_orgs: Vec<DbUserOrganization> = serde_path_to_error::deserialize(row.organizations)?;
    let organizations = user_orgs.into_iter().map(|uo| uo.into()).collect();
    let groups = serde_path_to_error::deserialize(row.groups)?;

    Ok(User {
        id: row.id.into(),
        name: row.name,
        email: row.email,
        last_sign_in: row.last_sign_in,
        created_at: row.created_at,
        updated_at: row.updated_at,
        profile_image: row.profile_image.map(|text| text.parse().unwrap()),
        organizations,
        groups,
    })
}

fn handle_group_row(row: GroupRowBorrowed) -> Result<Group> {
    let row: GroupRow = row.into();
    let users = serde_path_to_error::deserialize(row.users)?;
    let collections = serde_path_to_error::deserialize(row.collections)?;
    let price_lists = serde_path_to_error::deserialize(row.price_lists)?;
    Ok(Group {
        id: row.id.into(),
        name: row.name,
        description: row.description,
        created_at: row.created_at,
        created_by: row.created_by.map(|v| v.into()),
        updated_at: row.updated_at,
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
        users,
        collections,
        price_lists,
    })
}

fn handle_summary_group_row(row: GroupSummaryRowBorrowed) -> Result<GroupSummary> {
    let row: GroupSummaryRow = row.into();
    Ok(GroupSummary {
        id: row.id.into(),
        name: row.name,
        description: row.description,
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
        num_users: row.num_users as u32,
        num_collections: row.num_collections as u32,
        num_price_lists: row.num_price_lists as u32,
    })
}

#[derive(Debug, Deserialize)]
struct DbUserOrganization {
    pub organization: OrganizationSummary,
    pub role_ids: Vec<i32>,
}

impl From<DbUserOrganization> for UserOrganization {
    fn from(value: DbUserOrganization) -> Self {
        Self {
            organization: value.organization,
            roles: Role::from_db_ids(&value.role_ids),
        }
    }
}
