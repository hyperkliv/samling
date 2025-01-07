use jsonwebtoken::{
    jwk::{AlgorithmParameters, JwkSet},
    DecodingKey, Validation,
};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Error, Result};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCredentials {
    pub id_token: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GoogleClaims {
    pub(crate) email: String,
    pub(crate) email_verified: bool,
    pub(crate) picture: String,
    pub(crate) given_name: String,
    pub(crate) family_name: String,
}

impl GoogleClaims {
    pub(crate) async fn verify(audience: &[&str], login: &GoogleCredentials) -> Result<Self> {
        // TODO: Cache response for as long as cache-control header allows (22967 seconds currently)
        let resp = reqwest::get("https://www.googleapis.com/oauth2/v3/certs").await?;
        let jwks: JwkSet = resp.json().await?;
        let header = jsonwebtoken::decode_header(&login.id_token)
            .map_err(|err| Error::InvalidToken(err.to_string()))?;
        if let Some(kid) = header.kid {
            if let Some(jwk) = jwks.find(&kid) {
                let mut validation = Validation::new(header.alg);
                validation.set_audience(audience);
                validation.set_issuer(&["https://accounts.google.com"]);
                match &jwk.algorithm {
                    AlgorithmParameters::RSA(rsa) => {
                        let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                            .map_err(|err| Error::InvalidToken(err.to_string()))?;
                        let decoded = jsonwebtoken::decode::<GoogleClaims>(
                            &login.id_token,
                            &key,
                            &validation,
                        )
                        .map_err(|err| Error::InvalidToken(err.to_string()))?;
                        if decoded.claims.email_verified {
                            Ok(decoded.claims)
                        } else {
                            Err(Error::UnverifiedEmail(decoded.claims.email))
                        }
                    }
                    other => Err(Error::InvalidToken(format!(
                        "Google only supports RSA but got algorithm: {other:?}"
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
