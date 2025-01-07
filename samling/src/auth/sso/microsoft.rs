use bytes::Bytes;
use jsonwebtoken::{
    jwk::{AlgorithmParameters, JwkSet},
    DecodingKey, Validation,
};
use reqwest::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Error, Result};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MicrosoftCredentials {
    id_token_claims: MicrosoftClaims,
    pub id_token: String,
    pub access_token: String,
}

impl MicrosoftCredentials {
    pub(crate) fn unverified_id_token_claims(&self) -> &MicrosoftClaims {
        &self.id_token_claims
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(crate) struct MicrosoftClaims {
    // pub ver: String,
    // pub iss: String,
    // pub sub: String,
    // pub aud: String,
    // pub exp: DateTime<FixedOffset>,
    // pub iat: DateTime<FixedOffset>,
    // pub nbf: DateTime<FixedOffset>,
    pub name: String,
    // pub preferred_username: String,
    // pub oid: String,
    pub email: String,
    // pub tid: String,
    // pub nonce: String,
    // pub aio: String,
}

pub(crate) async fn get_profile_image(access_token: &str) -> Result<Option<Bytes>> {
    let request = reqwest::Client::new()
        .get("https://graph.microsoft.com/v1.0/me/photo/$value")
        .bearer_auth(access_token);
    let resp = request.send().await?;
    if resp.status() == StatusCode::NOT_FOUND {
        Ok(None)
    } else {
        resp.error_for_status_ref()?;
        Ok(Some(resp.bytes().await?))
    }
}

impl MicrosoftClaims {
    pub(crate) async fn verify(audience: &[&str], login: &MicrosoftCredentials) -> Result<Self> {
        // TODO: Cache response for as long as cache-control header allows (22967 seconds currently)
        let resp =
            reqwest::get("https://login.microsoftonline.com/common/discovery/v2.0/keys").await?;
        let jwks: JwkSet = resp.json().await?;
        let header = jsonwebtoken::decode_header(&login.id_token)
            .map_err(|err| Error::InvalidToken(err.to_string()))?;
        if let Some(kid) = header.kid {
            if let Some(jwk) = jwks.find(&kid) {
                let mut validation = Validation::new(header.alg);
                validation.set_audience(audience);
                match &jwk.algorithm {
                    AlgorithmParameters::RSA(rsa) => {
                        let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                            .map_err(|err| Error::InvalidToken(err.to_string()))?;
                        let decoded = jsonwebtoken::decode::<MicrosoftClaims>(
                            &login.id_token,
                            &key,
                            &validation,
                        )
                        .map_err(|err| Error::InvalidToken(err.to_string()))?;
                        Ok(decoded.claims)
                    }
                    other => Err(Error::InvalidToken(format!(
                        "Microsoft only supports RSA but got algorithm: {other:?}",
                    ))),
                }
            } else {
                Err(Error::InvalidToken("Failed to find token key".into()))
            }
        } else {
            Err(Error::InvalidToken("No `kid` value found".into()))
        }
    }
}
