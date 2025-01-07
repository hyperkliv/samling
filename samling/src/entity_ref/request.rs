use std::collections::HashMap;

use axum::{
    extract::{FromRequestParts, Path},
    RequestPartsExt,
};
use http::request::Parts;

use crate::{Error, Result};

use super::{Ref, RefTarget};

pub trait EntityRefPathParam {
    fn parameter_name() -> &'static str;
}

/// Extract entity_ref from HTTP path
impl<S, T> FromRequestParts<S> for Ref<T>
where
    S: Send + Sync,
    T: RefTarget + EntityRefPathParam,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let map = parts.extract::<Path<HashMap<String, String>>>().await?.0;
        if let Some(value) = map.get(T::parameter_name()) {
            Ok(Ref::<T>::parse(value.to_owned())?)
        } else {
            Err(Error::MissingEntityRefPathParameter(T::parameter_name()))
        }
    }
}
