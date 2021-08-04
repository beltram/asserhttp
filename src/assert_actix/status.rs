use actix_http::{body::Body as ActixBody, Response as ActixResponse, Error as ActixError};
use actix_web::dev::ServiceResponse as ActixServiceResponse;

use crate::asserter::status::{assert_status, assert_status_range};

use super::super::AsserhttpStatus;

impl AsserhttpStatus<ActixResponse<ActixBody>> for ActixResponse<ActixBody> {
    fn expect_status_eq(&mut self, status: u16) -> &mut Self {
        assert_status(self.status().as_u16(), status);
        self
    }

    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut Self {
        assert_status_range(self.status().as_u16(), lower, upper);
        self
    }
}

impl AsserhttpStatus<ActixResponse<ActixBody>> for Result<ActixResponse<ActixBody>, ActixError> {
    fn expect_status_eq(&mut self, status: u16) -> &mut ActixResponse<ActixBody> {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut ActixResponse<ActixBody> {
        self.as_mut().unwrap().expect_status_in_range(lower, upper)
    }
}

impl AsserhttpStatus<ActixServiceResponse<ActixBody>> for ActixServiceResponse<ActixBody> {
    fn expect_status_eq(&mut self, status: u16) -> &mut Self {
        assert_status(self.status().as_u16(), status);
        self
    }

    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut Self {
        assert_status_range(self.status().as_u16(), lower, upper);
        self
    }
}