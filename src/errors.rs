use axum::{
    extract::rejection::{PathRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use reqwest::header::InvalidHeaderValue;
use schemars::JsonSchema;
use serde::Serialize;

use crate::auth::rbac::Permission;

pub type Result<T> = std::result::Result<T, Error>;

// NOTE: Error variants are exposed to the frontend, so please name carefully!
//       One rule to follow is that no library names must be mentioned.
//
#[derive(thiserror::Error, strum::IntoStaticStr, Debug, strum::EnumDiscriminants)]
#[strum_discriminants(derive(JsonSchema, Serialize))]
#[strum_discriminants(name(ApiErrorCode))]
#[strum_discriminants(vis(pub))]
pub enum Error {
    #[error("Failed authentication")]
    InvalidUserCredentials,
    #[error("User with e-mail address {0} already exists")]
    UserEmailAlreadyExists(String),
    #[error("User e-mail {0} does not exist")]
    UserEmailNotFound(String),
    #[error("Empty slugs are disallowed")]
    EmptySlugDisallowed,
    #[error("Password hashing error: {0}")]
    FailedPasswordHashing(String),
    #[error("Password validation error: {0}")]
    FailedPasswordValidation(String),
    #[error("Invalid authentication token: {0}")]
    InvalidToken(String),
    #[error("The authentication token has expired")]
    ExpiredToken,
    #[error("E-mail address {0} has not been verified")]
    UnverifiedEmail(String),
    #[error("Role id {0} from DB does not exist")]
    InvalidDbRoleId(i32),
    #[error("Missing permissions: [{0:?}]")]
    MissingPermissions(Vec<Permission>),
    #[error("Application not ready")]
    ApplicationNotReady,
    #[error("Image uploads are currently unavailable")]
    ImageUploadsUnavailable,
    #[error("{0}")]
    PathRejection(#[from] PathRejection),
    #[error("{0}")]
    QueryParsingError(String),
    #[error("{0}")]
    QueryRejection(#[from] QueryRejection),
    #[error("reqwest error: {0}")]
    ExternalRequestError(#[from] reqwest::Error),
    #[error("{0}")]
    JsonError(#[from] serde_json::Error),
    #[error("{0}")]
    PathJsonError(#[from] serde_path_to_error::Error<serde_json::Error>),
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("csv: {0}")]
    CsvError(#[from] csv::Error),
    #[error("rust_xlsxwriter: {0}")]
    XlsxError(#[from] rust_xlsxwriter::XlsxError),
    #[error("base64: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("Invalid header error: {0}")]
    InvalidHttpHeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error("Invalid header error: {0}")]
    InvalidReqwestHttpHeaderValue(#[from] InvalidHeaderValue),
    #[error("tokio-postgres error: {0}")]
    DbError(#[from] tokio_postgres::Error),
    #[error("Deadpool pool error: {0}")]
    DbPoolError(#[from] deadpool_postgres::PoolError),
    #[error("Deadpool create pool error: {0}")]
    DbCreatePoolError(#[from] deadpool_postgres::CreatePoolError),
    #[error("Deadpool build error: {0}")]
    DbBuildError(#[from] deadpool_postgres::BuildError),
    #[error("{0} with id {1} was not found")]
    IdNotFound(&'static str, i32),
    #[error(r#"{0} with external id "{1}" was not found"#)]
    ExternalIdNotFound(&'static str, String),
    #[error(r#"{0} with slug "{1}" was not found"#)]
    SlugNotFound(&'static str, String),
    #[error("{0} with id {1} already exists")]
    IdAlreadyExists(&'static str, i32),
    #[error(r#"{0} with slug "{1}" already exists"#)]
    SlugAlreadyExists(&'static str, String),
    #[error(r#"{0} with external id "{1}" already exists"#)]
    ExternalIdAlreadyExists(&'static str, String),
    #[error("{0}")]
    InvalidEntityRef(String),
    #[error("Creating an entity with explicit ID is disallowed")]
    ExplicitIdCreationDisallowed,
    #[error("Entity type {0} does not support external ID references")]
    ExternalIdReferenceUnsupported(&'static str),
    #[error("Entity type {0} does not support slug references")]
    SlugReferenceUnsupported(&'static str),
    #[error("{0}")]
    ImageBackendMisconfigured(String),
    #[error("Image `{0}` already exists")]
    ImageAlreadyExists(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Missing entity ref path parameter `{0}`")]
    MissingEntityRefPathParameter(&'static str),
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ApiErrorResponse {
    error_code: ApiErrorCode,
    error_message: String,
}

impl ApiErrorResponse {
    fn new(error_code: ApiErrorCode, error_message: String) -> Self {
        Self {
            error_code,
            error_message,
        }
    }
}

use Error::*;

impl Error {
    pub(crate) fn user_facing(&self) -> bool {
        matches!(
            &self,
            InvalidUserCredentials
                | EmptySlugDisallowed
                | UserEmailAlreadyExists(..)
                | InvalidToken(..)
                | UnverifiedEmail(..)
                | ExpiredToken
                | MissingPermissions(..)
                | IdNotFound(..)
                | ExternalIdNotFound(..)
                | SlugNotFound(..)
                | IdAlreadyExists(..)
                | ExternalIdAlreadyExists(..)
                | SlugAlreadyExists(..)
                | ExplicitIdCreationDisallowed
                | ImageUploadsUnavailable
                | PathJsonError(..)
                | InvalidEntityRef(..)
                | ExternalIdReferenceUnsupported(..)
                | SlugReferenceUnsupported(..)
                | QueryRejection(..)
                | QueryParsingError(..)
                | JsonError(..)
                | PathRejection(..)
                | MissingEntityRefPathParameter(..)
        )
    }

    pub(crate) fn user_message(&self) -> String {
        match &self {
            InvalidUserCredentials => "Invalid user credentials".into(),
            err @ IdNotFound(..)
            | err @ ExternalIdNotFound(..)
            | err @ SlugNotFound(..)
            | err @ IdAlreadyExists(..)
            | err @ ExternalIdAlreadyExists(..)
            | err @ SlugAlreadyExists(..)
            | err @ EmptySlugDisallowed
            | err @ ExplicitIdCreationDisallowed
            | err @ ImageUploadsUnavailable
            | err @ PathJsonError(..)
            | err @ InvalidEntityRef(..)
            | err @ ExternalIdReferenceUnsupported(..)
            | err @ SlugReferenceUnsupported(..)
            | err @ QueryRejection(..)
            | err @ QueryParsingError(..)
            | err @ JsonError(..)
            | err @ PathRejection(..)
            | err @ MissingEntityRefPathParameter(..)
            | err @ MissingPermissions(..)
            | err @ UnverifiedEmail(..)
            | err @ ExpiredToken => err.to_string(),
            InvalidToken(..) => "Invalid token".into(),
            _ => "Internal server error".into(),
        }
    }
    pub(crate) fn status_code(&self) -> StatusCode {
        match &self {
            InvalidUserCredentials => StatusCode::UNAUTHORIZED,
            IdNotFound(..) | SlugNotFound(..) | ExternalIdNotFound(..) => StatusCode::NOT_FOUND,
            ExternalIdAlreadyExists(..) | SlugAlreadyExists(..) | IdAlreadyExists(..) => {
                StatusCode::CONFLICT
            }
            UserEmailAlreadyExists(..) => StatusCode::CONFLICT,
            ExpiredToken | UnverifiedEmail(..) | InvalidToken(..) => StatusCode::UNAUTHORIZED,
            MissingPermissions(..) => StatusCode::FORBIDDEN,
            ApplicationNotReady | ImageUploadsUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            ExternalIdReferenceUnsupported(..)
            | SlugReferenceUnsupported(..)
            | EmptySlugDisallowed
            | ExplicitIdCreationDisallowed
            | PathJsonError(..)
            | QueryRejection(..)
            | QueryParsingError(..)
            | JsonError(..)
            | PathRejection(..)
            | MissingEntityRefPathParameter(..)
            | InvalidEntityRef(..) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Log error with tracing and return self
    pub(crate) fn traced(self) -> Self {
        self.trace();
        self
    }

    /// Log error with tracing
    fn trace(&self) {
        if self.user_facing() {
            tracing::debug!("Got user facing error: {self:?}");
        } else {
            let msg = format!("Got non-user facing error: {self:?}");
            match self {
                ApplicationNotReady => {
                    tracing::info!(msg);
                }
                _ => {
                    tracing::error!(msg);
                }
            }
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_message = self.user_message();
        self.trace();

        let error_code: ApiErrorCode = self.into();

        let body = Json(ApiErrorResponse::new(error_code, error_message));

        (status, body).into_response()
    }
}

pub type CliResult<T> = std::result::Result<T, CliError>;

#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    AppError(#[from] Error),
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    DotenvError(#[from] dotenvy::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),
    #[error("{0}")]
    CornucopiaContainerError(String), // Unfortunately private
    #[error("Deadpool pool error: {0}")]
    DbPoolError(#[from] deadpool_postgres::PoolError),
    #[error("Must be in project root to run this command")]
    NotInProjectRoot,
}
