use super::{
    AsyncRocketResponse,
    RocketResponse,
    super::{
        AnyStatus,
        AsserhttpStatus,
        asserter::status::{assert_status, assert_status_range},
    },
};

impl<'a> AsserhttpStatus<RocketResponse<'a>> for RocketResponse<'a> {
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