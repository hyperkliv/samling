use std::{collections::HashMap, fmt::Debug};

use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Query},
    RequestPartsExt,
};
use deadpool_postgres::Object;
use http::request::Parts;
use serde::Deserialize;

use crate::{helpers, state::AppState, Error, Result};

pub(crate) struct PoolClient(pub(crate) Object);

/// Extract a database object ("connection") from AppState
#[async_trait]
impl FromRequestParts<AppState> for PoolClient {
    type Rejection = Error;

    async fn from_request_parts(_parts: &mut Parts, state: &AppState) -> Result<Self> {
        let client = state.db_pool.get().await?;
        Ok(PoolClient(client))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FiltersQuery<T>(pub(crate) T);

#[derive(Debug, Clone, Deserialize)]
struct RawFilters {
    filters: String,
}

#[async_trait]
impl<S, T> FromRequestParts<S> for FiltersQuery<T>
where
    S: Send + Sync,
    T: serde::de::DeserializeOwned + Default,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        match parts.extract::<Option<Query<RawFilters>>>().await {
            Ok(Some(Query(raw))) => {
                let output: T = serde_json::from_str(&raw.filters)?;
                Ok(Self(output))
            }
            _ => Ok(Self(T::default())),
        }
    }
}

/// Query containing JSON serialized values
pub(crate) struct JsonQuery<T>(pub(crate) T);

#[async_trait]
impl<S, T> FromRequestParts<S> for JsonQuery<T>
where
    S: Send + Sync,
    T: serde::de::DeserializeOwned,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let mut out: HashMap<String, serde_json::Value> = Default::default();
        let field_names = helpers::struct_fields::<T>();
        let parsed = parts.extract::<Query<Vec<(String, String)>>>().await?.0;
        for (param, val) in parsed {
            if field_names.contains(&param.as_str()) {
                let new_val = serde_json::from_str::<serde_json::Value>(&val).map_err(|err| {
                    Error::QueryParsingError(format!(
                        "`{param}` failed JSON deserialization with error: {err}"
                    ))
                })?;
                out.entry(param).or_insert(new_val);
            }
        }
        let new = serde_json::to_string(&out)?;
        let finished: T =
            serde_json::from_str(&new).map_err(|err| Error::QueryParsingError(err.to_string()))?;

        Ok(Self(finished))
    }
}
