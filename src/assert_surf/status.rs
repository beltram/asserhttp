use surf::{Error as SurfError, Response as SurfResponse};

use crate::asserter::status::{assert_status, assert_status_range};

use super::super::{AnyStatus, AsserhttpStatus};

impl AsserhttpStatus<SurfResponse> for SurfResponse {
    fn expect_status_eq<S: Into<AnyStatus>>(&mut self, status: S) -> &mut Self {
        assert_status(u16::from(self.status()), status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut Self {
        assert_status_range(u16::from(self.status()), lower, upper);
        self
    }
}

impl AsserhttpStatus<SurfResponse> for Result<SurfResponse, SurfError> {
    fn expect_status_eq<S: Into<AnyStatus>>(&mut self, status: S) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_status_in_range(lower, upper)
    }
}