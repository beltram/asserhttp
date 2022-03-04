use rocket::local::{
    asynchronous::LocalResponse as AsyncRocketResponse,
    blocking::LocalResponse as BlockingRocketResponse,
};

use crate::asserter::status::{assert_status, assert_status_range};

use super::super::{AnyStatus, AsserhttpStatus};

impl<'a> AsserhttpStatus<BlockingRocketResponse<'a>> for BlockingRocketResponse<'a> {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut Self {
        assert_status(self.status().code, status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut Self {
        assert_status_range(self.status().code, lower.into().0, upper.into().0);
        self
    }
}

impl<'a> AsserhttpStatus<AsyncRocketResponse<'a>> for AsyncRocketResponse<'a> {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut Self {
        assert_status(self.status().code, status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut Self {
        assert_status_range(self.status().code, lower.into().0, upper.into().0);
        self
    }
}