use async_trait::async_trait;
use cornucopia_async::{GenericClient, Params};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    cornucopia::queries::group::get_group_id,
    cornucopia::queries::user::{get_org_user_id, GetOrgUserIdParams},
    entity_ref::{Id, Ref, RefTarget},
    helpers::slugify,
    organizations::Organization,
    Collection, CollectionSummary, EntityRefPathParam, Environment, Error, ExternalId,
    ExternalIdEntity, OrganizationSummary, PriceList, PriceListSummary, Slug, SlugEntity,
};

use super::rbac::Role;

#[derive(Debug, Clone, Deserialize, Serialize, derive_more::Display, JsonSchema)]
#[display(fmt = "{id}/{name}")]
pub struct User {
    pub id: Id<User>,
    pub name: String,
    pub email: String,
    #[serde(with = "time::serde::rfc3339::option")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub last_sign_in: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
    pub profile_image: Option<Url>,
    pub organizations: Vec<UserOrganization>,
    pub groups: Vec<GroupSummary>,
}

#[derive(Debug, Clone, Deserialize, Serialize, derive_more::Display, JsonSchema)]
#[display(fmt = "{id}/{name}")]
pub struct UserSummary {
    pub id: Id<User>,
    pub name: String,
    pub email: String,
    #[serde(with = "time::serde::rfc3339::option")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub last_sign_in: Option<time::OffsetDateTime>,
    pub profile_image: Option<Url>,
}

#[async_trait]
impl RefTarget for User {
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        match entity_ref {
            Ref::Id(user_id) => Ok(get_org_user_id()
                .params(
                    client,
                    &GetOrgUserIdParams {
                        user_id: user_id.into(),
                        organization_id: organization_id.into(),
                    },
                )
                .opt()
                .await?
                .map(Id::new)),
            Ref::ExternalId(_) => Err(Error::ExternalIdReferenceUnsupported(
                Self::short_type_name(),
            )),
            Ref::Slug(_) => Err(Error::SlugReferenceUnsupported(Self::short_type_name())),
        }
    }
}

#[derive(
    Debug, Clone, Deserialize, Serialize, JsonSchema, derive_more::Display, derive_more::Constructor,
)]
#[display(fmt = "{organization} {roles:?}")]
pub struct UserOrganization {
    pub organization: OrganizationSummary,
    pub roles: Vec<Role>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserRoles {
    pub user: Id<User>,
    pub roles: Vec<Role>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Group {
    pub id: Id<Self>,
    pub slug: Slug<Self>,
    pub external_id: Option<ExternalId<Self>>,
    pub name: String,
    pub description: String,
    pub created_by: Option<Id<User>>,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
    pub users: Vec<UserSummary>,
    pub collections: Vec<CollectionSummary>,
    pub price_lists: Vec<PriceListSummary>,
}

impl EntityRefPathParam for Group {
    fn parameter_name() -> &'static str {
        "group_ref"
    }
}

#[async_trait]
impl RefTarget for Group {
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        let (id, external_id, slug) = entity_ref.to_owned().take_all_inner();
        Ok(get_group_id()
            .bind(
                client,
                &organization_id.into(),
                &id,
                &external_id.as_deref(),
                &slug.as_deref(),
            )
            .opt()
            .await?
            .map(Id::new))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GroupSummary {
    pub id: Id<Group>,
    pub slug: Slug<Group>,
    pub external_id: Option<ExternalId<Group>>,
    pub name: String,
    pub description: String,
    pub num_users: u32,
    pub num_collections: u32,
    pub num_price_lists: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct CreateGroup {
    pub slug: Option<Slug<Group>>,
    pub external_id: Option<ExternalId<Group>>,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub users: Vec<Ref<User>>,
    #[serde(default)]
    pub collections: Vec<Ref<Collection>>,
    #[serde(default)]
    pub price_lists: Vec<Ref<PriceList>>,
}

impl SlugEntity for CreateGroup {
    type RefTarget = Group;
    fn generate_slug(&self, prefix: &str) -> Option<Slug<Self::RefTarget>> {
        Some(Slug::new(slugify(&[prefix, &self.name])))
    }

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for CreateGroup {
    type RefTarget = Group;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct UpdateGroup {
    pub slug: Option<Slug<Group>>,
    pub external_id: Option<ExternalId<Group>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub users: Option<Vec<Ref<User>>>,
    pub collections: Option<Vec<Ref<Collection>>>,
    pub price_lists: Option<Vec<Ref<PriceList>>>,
}

impl SlugEntity for UpdateGroup {
    type RefTarget = Group;

    fn slug(&self) -> Option<Slug<Self::RefTarget>> {
        self.slug.clone()
    }

    fn set_slug(&mut self, value: Slug<Self::RefTarget>) {
        self.slug = Some(value);
    }
}

impl ExternalIdEntity for UpdateGroup {
    type RefTarget = Group;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

impl From<CreateGroup> for UpdateGroup {
    fn from(data: CreateGroup) -> Self {
        Self {
            name: Some(data.name),
            description: Some(data.description),
            slug: data.slug,
            external_id: data.external_id,
            users: Some(data.users),
            collections: Some(data.collections),
            price_lists: Some(data.price_lists),
        }
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CreateUser {
    pub email: String,
    pub password: Option<String>,
    pub name: String,
    pub profile_image: Option<Url>,
    pub roles: Option<Vec<Role>>,
    pub groups: Option<Vec<Ref<Group>>>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub profile_image: Option<Url>,
    pub roles: Option<Vec<Role>>,
    pub groups: Option<Vec<Ref<Group>>>,
}

impl From<CreateUser> for UpdateUser {
    fn from(user: CreateUser) -> Self {
        Self {
            email: Some(user.email),
            password: user.password,
            name: Some(user.name),
            profile_image: user.profile_image,
            roles: user.roles,
            groups: user.groups,
        }
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct UpdateOwnUser {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub profile_image: Option<Url>,
}

impl From<UpdateOwnUser> for UpdateUser {
    fn from(user: UpdateOwnUser) -> Self {
        Self {
            email: user.email,
            password: user.password,
            name: user.name,
            profile_image: user.profile_image,
            roles: None,
            groups: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

impl User {
    pub(crate) fn authenticated(
        self,
        environment: Environment,
        token: String,
    ) -> AuthenticatedUser {
        AuthenticatedUser {
            user: self,
            environment,
            token,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, derive_more::Constructor)]
pub struct AuthenticatedUser {
    pub user: User,
    pub environment: Environment,
    pub token: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_id_stringifies_to_just_integer_value() {
        assert_eq!(Id::<User>::new(123).to_string(), "123");
    }
}
