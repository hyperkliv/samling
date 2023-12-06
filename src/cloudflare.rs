use std::{num::NonZeroU32, time::Duration};

use base64::Engine;
use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use once_cell::sync::OnceCell;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    multipart::{self},
    StatusCode,
};
use serde::Deserialize;
use tracing::log::debug;
use url::Url;

use crate::{Error, ImageSource, ImageVariant, Result};

fn rate_limiter() -> &'static RateLimiter<NotKeyed, InMemoryState, DefaultClock> {
    static INSTANCE: OnceCell<RateLimiter<NotKeyed, InMemoryState, DefaultClock>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        // 1_200 requests per 5 minutes = 240 requests. Let's put it at 200 to be on the safe side!
        // https://community.cloudflare.com/t/cloudflare-images-rate-limit/361195/2
        let quota = Quota::per_minute(NonZeroU32::new(200).unwrap());
        RateLimiter::direct(quota)
    })
}

pub fn variant_url(url: &url::Url, variant: ImageVariant) -> url::Url {
    // NOTE: We've assumed that the URL is in Cloudflare Images format here and that
    //       the url ends with "/original" (which is the default on creation)
    debug_assert!(url.path().ends_with("/original"));
    let mut url = url.clone();
    {
        let mut path_segments = url.path_segments_mut().unwrap();
        path_segments.pop();
        path_segments.push(variant.snake_cased_name());
    }
    url
}

#[derive(Debug, Clone)]
pub struct CloudflareApi {
    client: reqwest::Client,
    base_url: Url,
    custom_images_domain: Option<String>,
}

impl CloudflareApi {
    pub fn new(
        account_id: String,
        token: String,
        custom_images_domain: Option<String>,
    ) -> Result<Self> {
        let mut default_headers = HeaderMap::new();
        default_headers.append(AUTHORIZATION, HeaderValue::from_str(&token)?);
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .default_headers(default_headers)
            .build()?;
        let base_url: Url = format!("https://api.cloudflare.com/client/v4/accounts/{account_id}/")
            .parse()
            .unwrap();
        Ok(Self {
            client,
            base_url,
            custom_images_domain,
        })
    }

    pub async fn upload_image(&self, id: String, source: ImageSource) -> Result<Url> {
        let original_id = id.clone();
        let mut res = Err(Error::ImageAlreadyExists(original_id.clone()));
        for retry_number in 0..100 {
            let id = if retry_number > 0 {
                let mut rng = rand::thread_rng();
                let suffix: String = (0..3).map(|_| rng.sample(Alphanumeric) as char).collect();
                format!("{original_id}-{suffix}")
            } else {
                original_id.clone()
            };
            match self.upload_image_impl(&id, &source).await {
                Ok(response) => {
                    let mut url = response.result.original_variant()?;
                    if let Some(custom_domain) = self.custom_images_domain.as_deref() {
                        url = apply_custom_domain(url, custom_domain)?;
                    }
                    res = Ok(url);
                    break;
                }
                Err(Error::ImageAlreadyExists(..)) => {
                    debug!("HTTP409 Conflict returned for image id {id:?}");
                    continue;
                }
                Err(err) => {
                    res = Err(err);
                    break;
                }
            }
        }
        res
    }

    async fn upload_image_impl(
        &self,
        id: &str,
        source: &ImageSource,
    ) -> Result<CloudflareImageUploadResponse> {
        let cf_url = self.base_url.join("images/v1").unwrap();
        let form = match source {
            ImageSource::Url(url) => multipart::Form::new()
                .text("url", url.to_string())
                .text("id", id.to_owned()),
            ImageSource::Bytes(data) => multipart::Form::new()
                .part("file", multipart::Part::bytes(data.to_vec()))
                .text("id", id.to_owned()),
            ImageSource::Base64(data) => {
                let engine = base64::engine::general_purpose::STANDARD;
                let decoded = engine.decode(data)?;
                multipart::Form::new()
                    .part("file", multipart::Part::bytes(decoded))
                    .text("id", id.to_owned())
            }
        };
        let ratelimiter = rate_limiter();
        ratelimiter.until_ready().await;
        let resp = self.client.post(cf_url).multipart(form).send().await?;
        match resp.error_for_status_ref() {
            Ok(_) => {
                let body = resp.text().await?;
                tracing::debug!("Cloudflare raw response body: {body}");
                Ok(serde_json::from_str::<CloudflareImageUploadResponse>(
                    &body,
                )?)
            }
            Err(err) => Err(if err.status() == Some(StatusCode::CONFLICT) {
                Error::ImageAlreadyExists(id.to_owned())
            } else {
                let body = resp.text().await?;
                tracing::error!("Cloudflare error response: {body}");
                err.into()
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct CloudflareImageUploadResponse {
    result: CloudflareImageUploadResult,
    // success: bool,
    // errors: serde_json::Value,
    // messages: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
struct CloudflareImageUploadResult {
    id: String,
    // filename: String,
    // uploaded: String,
    // requireSignedURLs: time::OffsetDateTime,
    variants: Vec<url::Url>,
}

impl CloudflareImageUploadResult {
    fn original_variant(&self) -> Result<url::Url> {
        for variant in &self.variants {
            if variant.path().ends_with("/original") {
                return Ok(variant.to_owned());
            }
        }
        Err(Error::ImageBackendMisconfigured(format!(
            "No `original` variant found for image {}. Is this variant not configured?",
            self.id
        )))
    }
}

fn apply_custom_domain(mut url: Url, custom_domain: &str) -> Result<Url> {
    url.set_host(Some(custom_domain))?;
    url.set_path(&format!("/cdn-cgi/imagedelivery{}", url.path()));
    Ok(url)
}
