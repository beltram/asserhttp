//! Assertions for gRPC client
//!
//! # Tonic
//!
//! ```no_run
//! # struct GrpcClient;
//! # impl GrpcClient {
//! #     async fn call_svc<T : Default>(&self, req: impl tonic::IntoRequest<T>) -> Result<tonic::Response<T>, tonic::Status> {
//! #         Ok(tonic::Response::new(T::default()))
//! #     }
//! # }
//! # #[derive(Debug, PartialEq, Default)]
//! # struct Payload;
//! use asserhttp::grpc::*;
//!
//! #[tokio::main]
//! async fn main() {
//! #    let client = GrpcClient;
//!     // success
//!     client.call_svc(tonic::Request::new(Payload)).await.expect_status_ok().expect_body(Payload);
//!     // error
//!     client.call_svc::<Payload>(tonic::Request::new(Payload)).await.expect_status_error(Code::NotFound);
//! }
//! ```
//!

use std::fmt::{Display, Formatter};

const STATUS_HEADER: &str = "grpc-status";

/// For assertions on successful gRPC responses
pub trait AssertGrpc<T>: AssertGrpcError {
    /// Expects response status to be 'OK' i.e. 0.
    /// See [this table](https://grpc.github.io/grpc/core/md_doc_statuscodes.html) for an exhaustive
    /// list of gRPC status codes
    ///
    /// # Example
    /// ```no_run
    /// # struct GrpcClient;
    /// # impl GrpcClient {
    /// #     async fn call_svc<T : Default>(&self, req: impl tonic::IntoRequest<T>) -> Result<tonic::Response<T>, tonic::Status> {
    /// #         Ok(tonic::Response::new(T::default()))
    /// #     }
    /// # }
    /// # #[derive(Debug, PartialEq, Default)]
    /// # struct Empty;
    /// #
    /// use asserhttp::grpc::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// #    let client = GrpcClient;
    ///     client.call_svc(tonic::Request::new(Empty)).await.expect_status_ok().expect_body(Empty);
    /// }
    /// ```
    #[must_use]
    fn expect_status_ok(self) -> tonic::Response<T>;
}

impl<T> AssertGrpc<T> for Result<tonic::Response<T>, tonic::Status> {
    fn expect_status_ok(self) -> tonic::Response<T> {
        match self.as_ref() {
            Ok(response) => {
                let actual = response
                    .metadata()
                    .get(STATUS_HEADER)
                    .map(|s| tonic::Code::from_bytes(s.as_bytes()))
                    .unwrap_or_else(|| panic!("Invalid gRPC response, no '{}' set", STATUS_HEADER));
                assert_eq!(
                    tonic::Code::Ok,
                    actual,
                    "expected status to be 'OK' but was '{}'",
                    Code::from(actual)
                );
            },
            Err(status) => {
                panic!("expected status to be 'OK' but was '{}'", Code::from(status.code()))
            },
        };
        self.unwrap()
    }
}

/// For assertions on unsuccessful gRPC responses
pub trait AssertGrpcError {
    /// Expects response status not to be 'OK', hence to be an error.
    /// See [this table](https://grpc.github.io/grpc/core/md_doc_statuscodes.html) for an exhaustive
    /// list of gRPC status codes
    ///
    /// * `status` - expected status
    ///
    /// # Example
    /// ```no_run
    /// # struct GrpcClient;
    /// # impl GrpcClient {
    /// #     async fn call_svc<T : Default>(&self, req: impl tonic::IntoRequest<T>) -> Result<tonic::Response<T>, tonic::Status> {
    /// #         Ok(tonic::Response::new(T::default()))
    /// #     }
    /// # }
    /// # #[derive(Debug, PartialEq, Default)]
    /// # struct Empty;
    /// #
    /// use asserhttp::grpc::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// #    let client = GrpcClient;
    ///     client.call_svc::<Empty>(tonic::Request::new(Empty)).await.expect_status_error(Code::NotFound);
    /// }
    /// ```
    fn expect_status_error(self, status: Code);
}

impl<T> AssertGrpcError for Result<tonic::Response<T>, tonic::Status> {
    fn expect_status_error(self, status: Code) {
        match self.as_ref() {
            Err(actual) => {
                let actual = Code::from(actual.code());
                let expected = status;
                assert_eq!(expected, actual, "expected status to be '{expected}' but was '{actual}'");
            },
            Ok(_) => panic!("expected status to be '{status}' but was 'OK'"),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Code {
    Cancelled = 1,
    Unknown = 2,
    InvalidArgument = 3,
    DeadlineExceeded = 4,
    NotFound = 5,
    AlreadyExists = 6,
    PermissionDenied = 7,
    ResourceExhausted = 8,
    FailedPrecondition = 9,
    Aborted = 10,
    OutOfRange = 11,
    Unimplemented = 12,
    Internal = 13,
    Unavailable = 14,
    DataLoss = 15,
    Unauthenticated = 16,
}

impl From<Code> for tonic::Code {
    fn from(code: Code) -> Self {
        match code {
            Code::Cancelled => Self::Cancelled,
            Code::Unknown => Self::Unknown,
            Code::InvalidArgument => Self::InvalidArgument,
            Code::DeadlineExceeded => Self::DeadlineExceeded,
            Code::NotFound => Self::NotFound,
            Code::AlreadyExists => Self::AlreadyExists,
            Code::PermissionDenied => Self::PermissionDenied,
            Code::ResourceExhausted => Self::ResourceExhausted,
            Code::FailedPrecondition => Self::FailedPrecondition,
            Code::Aborted => Self::Aborted,
            Code::OutOfRange => Self::OutOfRange,
            Code::Unimplemented => Self::Unimplemented,
            Code::Internal => Self::Internal,
            Code::Unavailable => Self::Unavailable,
            Code::DataLoss => Self::DataLoss,
            Code::Unauthenticated => Self::Unauthenticated,
        }
    }
}

impl From<tonic::Code> for Code {
    fn from(code: tonic::Code) -> Self {
        match code {
            tonic::Code::Cancelled => Self::Cancelled,
            tonic::Code::Unknown => Self::Unknown,
            tonic::Code::InvalidArgument => Self::InvalidArgument,
            tonic::Code::DeadlineExceeded => Self::DeadlineExceeded,
            tonic::Code::NotFound => Self::NotFound,
            tonic::Code::AlreadyExists => Self::AlreadyExists,
            tonic::Code::PermissionDenied => Self::PermissionDenied,
            tonic::Code::ResourceExhausted => Self::ResourceExhausted,
            tonic::Code::FailedPrecondition => Self::FailedPrecondition,
            tonic::Code::Aborted => Self::Aborted,
            tonic::Code::OutOfRange => Self::OutOfRange,
            tonic::Code::Unimplemented => Self::Unimplemented,
            tonic::Code::Internal => Self::Internal,
            tonic::Code::Unavailable => Self::Unavailable,
            tonic::Code::DataLoss => Self::DataLoss,
            tonic::Code::Unauthenticated => Self::Unauthenticated,
            tonic::Code::Ok => panic!("Implementation error"),
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Cancelled => "CANCELLED",
            Self::Unknown => "UNKNOWN",
            Self::InvalidArgument => "INVALID_ARGUMENT",
            Self::DeadlineExceeded => "DEADLINE_EXCEEDED",
            Self::NotFound => "NOT_FOUND",
            Self::AlreadyExists => "ALREADY_EXISTS",
            Self::PermissionDenied => "PERMISSION_DENIED",
            Self::ResourceExhausted => "RESOURCE_EXHAUSTED",
            Self::FailedPrecondition => "FAILED_PRECONDITION",
            Self::Aborted => "ABORTED",
            Self::OutOfRange => "OUT_OF_RANGE",
            Self::Unimplemented => "UNIMPLEMENTED",
            Self::Internal => "INTERNAL",
            Self::Unavailable => "UNAVAILABLE",
            Self::DataLoss => "DATA_LOSS",
            Self::Unauthenticated => "UNAUTHENTICATED",
        };
        write!(f, "{code}")
    }
}

/// For assertions on successful gRPC responses bodies
pub trait AssertGrpcBody<T: PartialEq + std::fmt::Debug> {
    /// Expects response body. This method should be called after [AssertGrpc::expect_status_ok].
    ///
    /// * `body` - expected response body
    ///
    /// # Example
    /// ```no_run
    /// # struct GrpcClient;
    /// # impl GrpcClient {
    /// #     async fn call_svc<T : Default>(&self, req: impl tonic::IntoRequest<T>) -> Result<tonic::Response<T>, tonic::Status> {
    /// #         Ok(tonic::Response::new(T::default()))
    /// #     }
    /// # }
    /// # #[derive(Debug, PartialEq, Default)]
    /// # struct Payload;
    /// #
    /// use asserhttp::grpc::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// #    let client = GrpcClient;
    ///     client.call_svc(tonic::Request::new(Payload)).await.expect_status_ok().expect_body(Payload);
    /// }
    /// ```
    fn expect_body(self, body: T);
}

impl<T: PartialEq + std::fmt::Debug> AssertGrpcBody<T> for tonic::Response<T> {
    fn expect_body(self, body: T) {
        let actual = self.into_inner();
        let expected = body;
        assert_eq!(
            expected, actual,
            "expected gRPC body to be:\n  {expected:?}\nbut was:\n  {actual:?}"
        );
    }
}
