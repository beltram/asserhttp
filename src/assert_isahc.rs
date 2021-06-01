use isahc::{
    AsyncBody as IsahcAsyncBody,
    Body as IsahcBody,
    Error as IsahcError,
    Response as IsahcResponse,
};

use super::{Asserhttp, TryAsserhttp};

impl Asserhttp for IsahcResponse<IsahcBody> {
    fn assert_status_eq(&mut self, status: u16) -> &mut Self {
        assert_eq!(self.status().as_u16(), status);
        self
    }
}

impl Asserhttp for IsahcResponse<IsahcAsyncBody> {
    fn assert_status_eq(&mut self, status: u16) -> &mut Self {
        assert_eq!(self.status().as_u16(), status);
        self
    }
}

impl TryAsserhttp<IsahcResponse<IsahcBody>> for Result<IsahcResponse<IsahcBody>, IsahcError> {
    fn assert_status_eq(&mut self, status: u16) -> &mut IsahcResponse<IsahcBody> {
        self.as_mut().unwrap().assert_status_eq(status)
    }
}

impl TryAsserhttp<IsahcResponse<IsahcAsyncBody>> for Result<IsahcResponse<IsahcAsyncBody>, IsahcError> {
    fn assert_status_eq(&mut self, status: u16) -> &mut IsahcResponse<IsahcAsyncBody> {
        self.as_mut().unwrap().assert_status_eq(status)
    }
}