use actix_http::{encoding::Decoder, Payload};
use awc::error::SendRequestError as AwcError;

use crate::asserter::status::{assert_status, assert_status_range};

use super::super::{AnyStatus, AsserhttpStatus};

pub type AwcResponse = awc::ClientResponse<Decoder<Payload>>;

impl AsserhttpStatus<AwcResponse> for AwcResponse {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut Self {
        assert_status(self.status().as_u16(), status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut Self {
        assert_status_range(self.status().as_u16(), lower.into().0, upper.into().0);
        self
    }
}

impl AsserhttpStatus<AwcResponse> for Result<AwcResponse, AwcError> {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_status_in_range(lower.into().0, upper.into().0)
    }
}