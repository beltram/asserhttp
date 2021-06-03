use isahc::{
    AsyncBody as IsahcAsyncBody,
    Body as IsahcBody,
    Error as IsahcError,
    Response as IsahcResponse,
};

use crate::asserter::status::assert_status;

use super::AsserhttpStatus;

impl AsserhttpStatus<IsahcResponse<IsahcBody>> for IsahcResponse<IsahcBody> {
    fn expect_status_eq(&mut self, status: u16) -> &mut Self {
        assert_status(self.status().as_u16(), status);
        self
    }
}

impl AsserhttpStatus<IsahcResponse<IsahcAsyncBody>> for IsahcResponse<IsahcAsyncBody> {
    fn expect_status_eq(&mut self, status: u16) -> &mut Self {
        assert_status(self.status().as_u16(), status);
        self
    }
}

impl AsserhttpStatus<IsahcResponse<IsahcBody>> for Result<IsahcResponse<IsahcBody>, IsahcError> {
    fn expect_status_eq(&mut self, status: u16) -> &mut IsahcResponse<IsahcBody> {
        self.as_mut().unwrap().expect_status_eq(status)
    }
}

impl AsserhttpStatus<IsahcResponse<IsahcAsyncBody>> for Result<IsahcResponse<IsahcAsyncBody>, IsahcError> {
    fn expect_status_eq(&mut self, status: u16) -> &mut IsahcResponse<IsahcAsyncBody> {
        self.as_mut().unwrap().expect_status_eq(status)
    }
}