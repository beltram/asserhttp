use crate::header::{key::HeaderKey, value::HeaderValue, values::HeaderValues};

pub type AsserhttpResult<T> = Result<T, AsserhttpError>;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "internal", derive(Eq, PartialEq))]
pub enum AsserhttpError {
    #[error("expected status to be '{expected}' but was '{actual}'")]
    StatusMismatch { actual: u16, expected: u16 },
    #[error("expected header '{key}' to be equal to '{expected}' but was '{actual}'")]
    HeaderValueMismatch {
        key: HeaderKey,
        actual: HeaderValue,
        expected: HeaderValue,
    },
    #[error("expected one header named '{key}' but none found")]
    HeaderAbsent { key: HeaderKey },
    #[error("expected no header named '{key}' but one found")]
    HeaderPresent { key: HeaderKey },
    #[error("expected header '{key}' to be single valued. Had '{values_count}' values '{actual_values}'. Use 'expect_headers' instead")]
    MultivaluedHeader {
        key: HeaderKey,
        values_count: usize,
        actual_values: HeaderValues,
    },
    #[error("no value expected for header '{key}'. Use 'expect_header_present' instead")]
    InvalidHeaderValuesSupplied { key: HeaderKey },
    #[error("expected header '{key}' to contain values '{expected}' but contained '{actual_values}'")]
    HeaderValuesMismatch {
        key: HeaderKey,
        actual_values: HeaderValues,
        expected: HeaderValues,
    },
    #[error("{0}")]
    JsonBodyMismatch(String),
    #[error("expected body to be '{expected}' but was '{actual}'")]
    TextBodyMismatch { actual: String, expected: String },
    #[error("expected body to be '{expected:?}' but was '{actual:?}'")]
    BytesBodyMismatch { actual: Vec<u8>, expected: Vec<u8> },
    #[error("expected body to match regex '{regex}' but was '{actual}'")]
    RegexBodyMismatch { actual: String, regex: String },
    #[error("expected a response body but none was present")]
    BodyAbsent,
    #[error("expected no response body but one was present")]
    BodyPresent,
    #[error("Error in the http client: {0}")]
    HttpError(String),
    #[error("{0}")]
    RegexError(String),
    #[error("Internal asserhttp error")]
    InternalError,
    #[cfg(not(feature = "internal"))]
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[cfg(not(feature = "internal"))]
    #[error(transparent)]
    HttpTypesError(anyhow::Error),
    #[cfg(not(feature = "internal"))]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(not(feature = "internal"))]
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[cfg(not(feature = "internal"))]
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[cfg(all(not(feature = "internal"), feature = "awc"))]
    #[error(transparent)]
    AwcError(#[from] actix_http::error::PayloadError),
    #[cfg(all(not(feature = "internal"), feature = "reqwest"))]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Error in a third party crate: {0}")]
    ExternalError(String),
}

#[cfg(feature = "internal")]
impl From<anyhow::Error> for AsserhttpError {
    fn from(e: anyhow::Error) -> Self {
        Self::ExternalError(e.to_string())
    }
}

#[cfg(not(feature = "internal"))]
impl From<http_types::Error> for AsserhttpError {
    fn from(e: http_types::Error) -> Self {
        Self::HttpTypesError(e.into_inner())
    }
}

#[cfg(feature = "internal")]
impl From<http_types::Error> for AsserhttpError {
    fn from(e: http_types::Error) -> Self {
        Self::ExternalError(e.to_string())
    }
}

#[cfg(feature = "internal")]
impl From<serde_json::Error> for AsserhttpError {
    fn from(e: serde_json::Error) -> Self {
        Self::ExternalError(e.to_string())
    }
}

#[cfg(feature = "internal")]
impl From<std::io::Error> for AsserhttpError {
    fn from(e: std::io::Error) -> Self {
        Self::ExternalError(e.to_string())
    }
}

#[cfg(feature = "internal")]
impl From<std::str::Utf8Error> for AsserhttpError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::ExternalError(e.to_string())
    }
}

#[cfg(all(feature = "internal", feature = "actix"))]
impl From<actix_http::error::PayloadError> for AsserhttpError {
    fn from(e: actix_http::error::PayloadError) -> Self {
        Self::ExternalError(e.to_string())
    }
}

#[cfg(all(feature = "internal", feature = "reqwest"))]
impl From<reqwest::Error> for AsserhttpError {
    fn from(e: reqwest::Error) -> Self {
        Self::ExternalError(e.to_string())
    }
}

#[cfg(feature = "axum")]
impl From<axum::Error> for AsserhttpError {
    fn from(e: axum::Error) -> Self {
        Self::ExternalError(e.to_string())
    }
}

impl From<regex::Error> for AsserhttpError {
    fn from(e: regex::Error) -> Self {
        Self::RegexError(e.to_string())
    }
}
