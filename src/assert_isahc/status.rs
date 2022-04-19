use super::{
    AsyncIsahcResponse,
    IsahcResponse,
    ResultAsyncIsahcResponse,
    ResultIsahcResponse,
    super::{
        AnyStatus,
        AsserhttpStatus,
        asserter::status::{assert_status, assert_status_range},
    },
};

impl AsserhttpStatus<IsahcResponse> for IsahcResponse {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut Self {
        assert_status(self.status().as_u16(), status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut Self {
        assert_status_range(self.status().as_u16(), lower.into().0, upper.into().0);
        self
    }
}

impl AsserhttpStatus<IsahcResponse> for ResultIsahcResponse {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut IsahcResponse {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut IsahcResponse {
        self.as_mut().unwrap().expect_status_in_range(lower.into().0, upper.into().0)
    }
}

impl AsserhttpStatus<AsyncIsahcResponse> for AsyncIsahcResponse {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut Self {
        assert_status(self.status().as_u16(), status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut Self {
        assert_status_range(self.status().as_u16(), lower.into().0, upper.into().0);
        self
    }
}

impl AsserhttpStatus<AsyncIsahcResponse> for ResultAsyncIsahcResponse {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut AsyncIsahcResponse {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut AsyncIsahcResponse {
        self.as_mut().unwrap().expect_status_in_range(lower.into().0, upper.into().0)
    }
}