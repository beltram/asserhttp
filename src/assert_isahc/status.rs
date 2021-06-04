use isahc::{AsyncBody as IsahcAsyncBody, AsyncBody, Body as IsahcBody, Body, Error as IsahcError, Response as IsahcResponse, Response};

use crate::asserter::status::{assert_status, assert_status_range};

use super::super::AsserhttpStatus;

impl AsserhttpStatus<IsahcResponse<IsahcBody>> for IsahcResponse<IsahcBody> {
    fn expect_status_eq(&mut self, status: u16) -> &mut Self {
        assert_status(self.status().as_u16(), status);
        self
    }

    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut Response<Body> {
        assert_status_range(self.status().as_u16(), lower, upper);
        self
    }
}

impl AsserhttpStatus<IsahcResponse<IsahcAsyncBody>> for IsahcResponse<IsahcAsyncBody> {
    fn expect_status_eq(&mut self, status: u16) -> &mut Self {
        assert_status(self.status().as_u16(), status);
        self
    }

    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut Response<AsyncBody> {
        assert_status_range(self.status().as_u16(), lower, upper);
        self
    }
}

impl AsserhttpStatus<IsahcResponse<IsahcBody>> for Result<IsahcResponse<IsahcBody>, IsahcError> {
    fn expect_status_eq(&mut self, status: u16) -> &mut IsahcResponse<IsahcBody> {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut Response<Body> {
        self.as_mut().unwrap().expect_status_in_range(lower, upper)
    }
}

impl AsserhttpStatus<IsahcResponse<IsahcAsyncBody>> for Result<IsahcResponse<IsahcAsyncBody>, IsahcError> {
    fn expect_status_eq(&mut self, status: u16) -> &mut IsahcResponse<IsahcAsyncBody> {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut Response<AsyncBody> {
        self.as_mut().unwrap().expect_status_in_range(lower, upper)
    }
}