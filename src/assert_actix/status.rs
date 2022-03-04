use actix_http::Error as ActixError;
use actix_web::{dev::ServiceResponse as ActixServiceResponse, HttpResponse as ActixResponse};

use super::super::{AnyStatus, AsserhttpStatus, asserter::status::{assert_status, assert_status_range}};

impl<B> AsserhttpStatus<ActixResponse<B>> for ActixResponse<B> {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut Self {
        assert_status(self.status().as_u16(), status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut Self {
        assert_status_range(self.status().as_u16(), lower.into().0, upper.into().0);
        self
    }
}

impl<B> AsserhttpStatus<ActixResponse<B>> for Result<ActixResponse<B>, ActixError> {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut ActixResponse<B> {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut ActixResponse<B> {
        self.as_mut().unwrap().expect_status_in_range(lower.into().0, upper.into().0)
    }
}

impl<B> AsserhttpStatus<ActixServiceResponse<B>> for ActixServiceResponse<B> {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut Self {
        assert_status(self.status().as_u16(), status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut Self {
        assert_status_range(self.status().as_u16(), lower.into().0, upper.into().0);
        self
    }
}