use std::fmt;

use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use http::request::Parts;
use itertools::Itertools;
use jsonwebtoken::{
    errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::{
    entity_ref::Id, state::AppState, Error, Organization, Permission, RequestMetaData, Result, Role,
};

use super::User;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TokenOrganizationRoles {
    #[serde(rename = "o")]
    organization_id: Id<Organization>,
    #[serde(rename = "r")]
    roles: Vec<Role>,
}

#[derive(Debug, Clone)]
pub struct JwtSigner {
    keys: Keys,
    token_ttl: u32,
    header: Header,
    validation: Validation,
}

impl JwtSigner {
    pub fn new(secret: &str) -> Self {
        let alg = Algorithm::HS256;
        Self {
            keys: Keys::new(secret.as_bytes()),
            token_ttl: 7 * 24 * 3600, // 7 days
            header: Header::new(alg),
            validation: Validation::new(alg),
        }
    }

    pub fn with_ttl(mut self, token_ttl: u32) -> Self {
        self.token_ttl = token_ttl;
        self
    }

    pub(crate) fn decode(&self, token: &str) -> Result<TokenData<Claims>> {
        jsonwebtoken::decode::<Claims>(token, &self.keys.decoding, &self.validation).map_err(
            |err| {
                if err.kind() == &ErrorKind::ExpiredSignature {
                    Error::ExpiredToken
                } else {
                    let s = err.to_string();
                    //
                    if s.contains("missing field `or`") {
                        // This is a special case after introduction of
                        // or / organization_roles.
                        Error::ExpiredToken
                    } else {
                        Error::InvalidToken(s)
                    }
                }
            },
        )
    }

    pub fn claims(&self, user: &User) -> Result<Claims> {
        Claims::new(self.token_ttl, user)
    }

    pub(crate) fn encode(&self, claims: Claims) -> Result<String> {
        let token = jsonwebtoken::encode(&self.header, &claims, &self.keys.encoding)
            .map_err(|err| Error::InvalidToken(err.to_string()))?;
        Ok(token)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub(crate) sub: Id<User>,
    pub(crate) exp: u64,
    pub(crate) iat: u64,
    #[serde(rename = "or")]
    pub(crate) organization_roles: Vec<TokenOrganizationRoles>,
}

impl Claims {
    pub(crate) fn new(token_ttl: u32, user: &User) -> Result<Self> {
        let now = std::time::SystemTime::now();
        let iat = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let exp = (now + Duration::seconds(token_ttl.into()))
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();

        Ok(Self {
            sub: user.id,
            iat: iat.as_secs(),
            exp: exp.as_secs(),
            organization_roles: user
                .organizations
                .iter()
                .map(|user_org| TokenOrganizationRoles {
                    organization_id: user_org.organization.id,
                    roles: user_org.roles.clone(),
                })
                .collect(),
        })
    }

    pub(crate) fn user_id(&self) -> Id<User> {
        self.sub
    }
    pub(crate) fn ensure(
        &self,
        organization_id: Id<Organization>,
        permissions: &[Permission],
    ) -> Result<RequestMetaData> {
        let org_permissions = self
            .organization_roles
            .iter()
            .find(|or| or.organization_id == organization_id)
            .map(|org_roles| Permission::from_roles(&org_roles.roles))
            .unwrap_or_default();

        let missing = permissions
            .iter()
            .filter(|p| !org_permissions.contains(p))
            .copied()
            .collect_vec();

        if !org_permissions.contains(&Permission::SignIn) {
            Err(Error::MissingPermissions(vec![Permission::SignIn]))
        } else if !missing.is_empty() {
            Err(Error::MissingPermissions(missing))
        } else {
            Ok(RequestMetaData::new(self, organization_id))
        }
    }
}

#[derive(Clone)]
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl fmt::Debug for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Keys")
            .field("encoding", &"<REDACTED>")
            .field("decoding", &"REDACTED")
            .finish()
    }
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[async_trait]
impl FromRequestParts<AppState> for Claims {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| Error::InvalidToken("Failed to parse header".into()))?;

        let token_data = state.jwt_signer.decode(bearer.token())?;

        Ok(token_data.claims)
    }
}
