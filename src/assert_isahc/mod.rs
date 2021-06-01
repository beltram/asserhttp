use isahc::{
    AsyncBody as IsahcAsyncBody,
    Body as IsahcBody,
    Error as IsahcError,
    Response as IsahcResponse,
};

use super::{
    Asserhttp,
    AsserhttpStatus,
    TryAsserhttp,
    TryAsserhttpStatus,
};

mod status;

impl Asserhttp for IsahcResponse<IsahcBody> {}

impl Asserhttp for IsahcResponse<IsahcAsyncBody> {}

impl TryAsserhttp<IsahcResponse<IsahcBody>> for Result<IsahcResponse<IsahcBody>, IsahcError> {}

impl TryAsserhttp<IsahcResponse<IsahcAsyncBody>> for Result<IsahcResponse<IsahcAsyncBody>, IsahcError> {}