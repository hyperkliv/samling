use bytes::Bytes;
use reqwest::Url;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{signing::Claims, Id, Organization, User};

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, strum::EnumString, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Environment {
    Production,
    Staging,
    Development,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ImageSource {
    Url(Url),
    Bytes(Bytes),
    Base64(String),
}

impl From<Url> for ImageSource {
    fn from(url: Url) -> Self {
        ImageSource::Url(url)
    }
}

impl From<Bytes> for ImageSource {
    fn from(data: Bytes) -> Self {
        ImageSource::Bytes(data)
    }
}

/// Information about which organization data is requested for, and by whom
#[derive(Debug, Clone, Copy)]
pub struct RequestMetaData {
    user_id: Id<User>,
    organization_id: Id<Organization>,
}

impl RequestMetaData {
    pub fn new(claims: &Claims, organization_id: Id<Organization>) -> Self {
        Self {
            user_id: claims.user_id(),
            organization_id,
        }
    }

    pub fn user_id(&self) -> Id<User> {
        self.user_id
    }

    pub fn organization_id(&self) -> Id<Organization> {
        self.organization_id
    }
}
