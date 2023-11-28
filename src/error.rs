use crate::header::{key::HeaderKey, value::HeaderValue, values::HeaderValues};

pub type AsserhttpResult<T> = Result<T, AsserhttpError>;

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
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
    #[error("expected header '{key}' to be single valued. Had '{values_count}' values '{actual_values:?}'. Use 'expect_headers' instead")]
    MultivaluedHeader {
        key: HeaderKey,
        values_count: usize,
        actual_values: HeaderValues,
    },
    #[error("no value expected for header '{key}'. Use 'expect_header_present' instead")]
    InvalidHeaderValuesSupplied { key: HeaderKey },
    #[error("expected header '{key}' to contain values '{expected:?}' but contained '{actual_values}'")]
    HeaderValuesMismatch {
        key: HeaderKey,
        actual_values: HeaderValues,
        expected: HeaderValues,
    },
    #[error("Error in the http client: {0}")]
    HttpError(String),
    #[error("Internal asserhttp error")]
    InternalError,
}
