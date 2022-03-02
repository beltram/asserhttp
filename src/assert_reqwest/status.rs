use reqwest::{blocking::Response as ReqwestResponse, Error as ReqwestError, Response as AsyncReqwestResponse};

use crate::asserter::status::{assert_status, assert_status_range};

use super::super::{AnyStatus, AsserhttpStatus};

impl AsserhttpStatus<ReqwestResponse> for ReqwestResponse {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut Self {
        assert_status(self.status().as_u16(), status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut Self {
        assert_status_range(self.status().as_u16(), lower.into().0, upper.into().0);
        self
    }
}

impl AsserhttpStatus<AsyncReqwestResponse> for AsyncReqwestResponse {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut Self {
        assert_status(self.status().as_u16(), status.into().0);
        self
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut Self {
        assert_status_range(self.status().as_u16(), lower.into().0, upper.into().0);
        self
    }
}

impl AsserhttpStatus<ReqwestResponse> for Result<ReqwestResponse, ReqwestError> {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut ReqwestResponse {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut ReqwestResponse {
        self.as_mut().unwrap().expect_status_in_range(lower.into().0, upper.into().0)
    }
}

impl AsserhttpStatus<AsyncReqwestResponse> for Result<AsyncReqwestResponse, ReqwestError> {
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut AsyncReqwestResponse {
        self.as_mut().unwrap().expect_status_eq(status)
    }

    fn expect_status_in_range(&mut self, lower: impl Into<AnyStatus>, upper: impl Into<AnyStatus>) -> &mut AsyncReqwestResponse {
        self.as_mut().unwrap().expect_status_in_range(lower.into().0, upper.into().0)
    }
}
