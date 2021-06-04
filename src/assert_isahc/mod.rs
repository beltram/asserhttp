use isahc::{
    AsyncBody as IsahcAsyncBody,
    Body as IsahcBody,
    Error as IsahcError,
    Response as IsahcResponse,
};

use super::Asserhttp;

mod status;
mod header;

impl Asserhttp<IsahcResponse<IsahcBody>> for IsahcResponse<IsahcBody> {}

impl Asserhttp<IsahcResponse<IsahcAsyncBody>> for IsahcResponse<IsahcAsyncBody> {}

impl Asserhttp<IsahcResponse<IsahcBody>> for Result<IsahcResponse<IsahcBody>, IsahcError> {}

impl Asserhttp<IsahcResponse<IsahcAsyncBody>> for Result<IsahcResponse<IsahcAsyncBody>, IsahcError> {}