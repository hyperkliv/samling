use chrono::{DateTime, FixedOffset};
use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    auth::User,
    entity_ref::{Id, RefTarget},
    ImageSource,
};
use samling_clorinde::{client::GenericClient, queries::organization::get_organization_id};

/// Organization
#[derive(Debug, Serialize, Deserialize, Clone, Display, JsonSchema)]
#[display("{id}/{name}")]
pub struct Organization {
    pub id: Id<Self>,
    pub name: String,
    pub created_by: Option<Id<User>>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
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
