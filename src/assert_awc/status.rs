use actix_http::{encoding::Decoder, Payload};
use awc::error::SendRequestError as AwcError;

use crate::asserter::status::{assert_status, assert_status_range};

use super::super::AsserhttpStatus;

pub type AwcResponse = awc::ClientResponse<Decoder<Payload>>;

impl AsserhttpStatus<AwcResponse> for AwcResponse {
    fn expect_status_eq(&mut self, status: u16) -> &mut Self {
        assert_status(self.status().as_u16(), status);
        self
    }

    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut Self {
        assert_status_range(self.status().as_u16(), lower, upper);
        self
    }
}

impl AsserhttpStatus<AwcResponse> for Result<AwcResponse, AwcError> {
    fn expect_status_eq(&mut self, status: u16) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_status_in_range(lower, upper)
    }
}