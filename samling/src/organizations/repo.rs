use samling_clorinde::{
    client::{GenericClient, Params},
    queries::organization::{
        delete_organization, get_organization, insert_organization, update_organization,
        InsertOrganizationParams, OrganizationRow, OrganizationRowBorrowed,
        UpdateOrganizationParams,
    },
};

use super::{CreateOrganization, Organization, UpdateOrganization};
use crate::{
    auth::User,
    entity_ref::{Id, RefType},
    helpers::slugify,
    CloudflareApi, Result,
};

#[derive(Clone)]
pub struct OrgRepo;

impl OrgRepo {
    async fn find(
        &self,
        client: &impl GenericClient,
        id: Id<Organization>,
    ) -> Result<Option<Organization>> {
        Ok(get_organization()
            .bind(client, &id.into())
            .map(handle_organization_row)
            .opt()
            .await?)
    }

    pub async fn get(
        &self,
        client: &impl GenericClient,
        id: Id<Organization>,
    ) -> Result<Organization> {
        if let Some(user) = self.find(client, id).await? {
            Ok(user)
        } else {
            Err(id.not_found_error())
        }
    }

    pub async fn insert(
        &self,
        client: &impl GenericClient,
        cloudflare_api: CloudflareApi,
        organization: CreateOrganization,
        user_id: Id<User>,
    ) -> Result<Organization> {
        let logo_url = if let Some(image_source) = organization.logo {
            Some(
                cloudflare_api
                    .upload_image(slugify(&[&organization.name]), image_source)
                    .await?,
            )
        } else {
            None
        };
        Ok(insert_organization()
            .params(
                client,
                &InsertOrganizationParams {
                    name: organization.name,
                    created_by: user_id.into(),
                    logo_url: logo_url.map(|url| url.to_string()),
                },
            )
            .map(handle_organization_row)
            .one()
            .await?)
    }

    pub async fn update(
        &self,
        client: &impl GenericClient,
        cloudflare_api: CloudflareApi,
        id: Id<Organization>,
        organization: UpdateOrganization,
    ) -> Result<Organization> {
        if let Some(existing) = self.find(client, id).await? {
            let logo_url = if let Some(image_source) = organization.logo {
                Some(
                    cloudflare_api
                        .upload_image(
                            slugify(&[organization.name.as_ref().unwrap_or(&existing.name)]),
                            image_source,
                        )
                        .await?,
                )
            } else {
                None
            };
            update_organization()
                .params(
                    client,
                    &UpdateOrganizationParams {
                        id: id.into(),
                        name: organization.name,
                        logo_url: logo_url.map(|url| url.to_string()),
                    },
                )
                .map(handle_organization_row)
                .opt()
                .await?;
            Ok(self.get(client, id).await?)
        } else {
            Err(id.not_found_error())
        }
    }

    pub async fn delete(&self, client: &impl GenericClient, id: Id<Organization>) -> Result<()> {
        if delete_organization()
            .bind(client, id.inner())
            .opt()
            .await?
            .is_some()
        {
            Ok(())
        } else {
            Err(id.not_found_error())
        }
    }
}

pub(crate) fn handle_organization_row(row: OrganizationRowBorrowed) -> Organization {
    let row: OrganizationRow = row.into();
    Organization {
        id: row.id.into(),
        name: row.name,
        logo_url: row.logo_url.map(|url| url.parse().unwrap()),
        created_by: row.created_by.map(|v| v.into()),
        updated_at: row.updated_at,
        created_at: row.created_at,
    }
}
