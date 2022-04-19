use super::{
    ActixResponse, ActixServiceResponse, ResultActixResponse,
    super::{AnyStatus, AsserhttpStatus, asserter::status::{assert_status, assert_status_range}}
};

impl AsserhttpStatus<ActixResponse> for ActixResponse {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut Self {
        assert_status(self.status().as_u16(), status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut Self {
        assert_status_range(self.status().as_u16(), lower.into().0, upper.into().0);
        self
    }
}

impl AsserhttpStatus<ActixResponse> for ResultActixResponse {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut ActixResponse {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut ActixResponse {
        self.as_mut().unwrap().expect_status_in_range(lower.into().0, upper.into().0)
    }
}

impl AsserhttpStatus<ActixServiceResponse> for ActixServiceResponse {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut Self {
        assert_status(self.status().as_u16(), status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut Self {
        assert_status_range(self.status().as_u16(), lower.into().0, upper.into().0);
        self
    }
}