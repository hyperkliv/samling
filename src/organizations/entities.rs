use async_trait::async_trait;
use cornucopia_async::GenericClient;
use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    auth::User,
    cornucopia::queries::organization::get_organization_id,
    entity_ref::{Id, RefTarget},
    ImageSource,
};

/// Organization
#[derive(Debug, Serialize, Deserialize, Clone, Display, JsonSchema)]
#[display("{id}/{name}")]
pub struct Organization {
    pub id: Id<Self>,
    pub name: String,
    pub created_by: Option<Id<User>>,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
    pub logo_url: Option<Url>,
}

/// Organization
#[derive(Debug, Serialize, Deserialize, Clone, Display, JsonSchema)]
#[display("{id}/{name}")]
pub struct OrganizationSummary {
    pub id: Id<Organization>,
    pub name: String,
    pub logo_url: Option<Url>,
}

#[async_trait]
impl RefTarget for Organization {
    async fn lookup_id(
        client: &impl GenericClient,
        _: Id<Self>,
        entity_ref: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        if let Some(id) = entity_ref.id() {
            Ok(get_organization_id()
                .bind(client, &id.into())
                .opt()
                .await?
                .map(Id::new))
        } else {
            Err(entity_ref.not_found_error())
        }
    }
}

/// Organization, for creation
#[derive(Debug, Deserialize)]
pub struct CreateOrganization {
    pub name: String,
    pub logo: Option<ImageSource>,
}

/// Organization, for update
#[derive(Debug, Deserialize)]
pub struct UpdateOrganization {
    pub name: Option<String>,
    pub logo: Option<ImageSource>,
}
