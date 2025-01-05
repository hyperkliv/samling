use cornucopi_async::GenericClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    auth::User,
    cornucopia::queries::image::get_image_id,
    entity_ref::{ExternalId, ExternalIdEntity},
    organizations::Organization,
    Color, ColorSummary, EntityRefPathParam, Id, Ref, RefTarget,
};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ImageVariant {
    Original,
    Medium,
    Thumbnail,
}

impl ImageVariant {
    pub(crate) fn snake_cased_name(&self) -> &'static str {
        match self {
            Self::Original => "original",
            Self::Medium => "medium",
            Self::Thumbnail => "thumbnail",
        }
    }
}

/// Image
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Image {
    pub id: Id<Self>,
    pub color: ColorSummary,
    pub url: url::Url,
    pub external_checksum: Option<String>,
    pub external_id: Option<ExternalId<Self>>,
    pub uploaded_by: Option<Id<User>>,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub uploaded_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schemars(schema_with = "crate::jsonschema::rfc3339_date_time")]
    pub updated_at: time::OffsetDateTime,
    pub position: i32,
}

impl EntityRefPathParam for Image {
    fn parameter_name() -> &'static str {
        "image_ref"
    }
}

/// Image summary
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ImageSummary {
    pub id: Id<Image>,
    pub url: url::Url,
    pub external_id: Option<ExternalId<Image>>,
}

impl RefTarget for Image {
    async fn lookup_id(
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        entity_ref: &crate::entity_ref::Ref<Self>,
    ) -> crate::Result<Option<Id<Self>>> {
        let (id, external_id, _) = entity_ref.to_owned().take_all_inner();
        Ok(get_image_id()
            .bind(
                client,
                &organization_id.into(),
                &id,
                &external_id.as_deref(),
            )
            .opt()
            .await?
            .map(Id::new))
    }
}

/// Image, for upload
#[derive(Debug, Clone, Deserialize)]
pub struct UploadSourceImage {
    pub source_url: Url,
    pub color: Ref<Color>,
    /// An optional checksum. If it's provided this will be used to avoid unnecessary uploads.
    pub external_checksum: Option<String>,
    pub external_id: Option<ExternalId<Image>>,
    #[serde(default)]
    pub position: Option<i32>,
}

impl ExternalIdEntity for UploadSourceImage {
    type RefTarget = Image;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}

/// Image, for creation
#[derive(Debug, Clone, Deserialize)]
pub struct UploadedImage {
    pub url: url::Url,
    pub color: Ref<Color>,
    pub external_id: Option<ExternalId<Image>>,
}

impl ExternalIdEntity for UploadedImage {
    type RefTarget = Image;

    fn external_id(&self) -> Option<ExternalId<Self::RefTarget>> {
        self.external_id.clone()
    }

    fn set_external_id(&mut self, value: ExternalId<Self::RefTarget>) {
        self.external_id = Some(value);
    }
}
