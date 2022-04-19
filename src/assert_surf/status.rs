use super::{
    ResultSurfResponse,
    super::{
        AnyStatus,
        AsserhttpStatus,
        asserter::status::{assert_status, assert_status_range},
    },
    SurfResponse,
};

impl AsserhttpStatus<SurfResponse> for SurfResponse {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut Self {
        assert_status(u16::from(self.status()), status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut Self {
        assert_status_range(u16::from(self.status()), lower.into().0, upper.into().0);
        self
    }
}

impl AsserhttpStatus<SurfResponse> for ResultSurfResponse {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_status_in_range(lower.into().0, upper.into().0)
    }
}