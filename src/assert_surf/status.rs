use surf::{Error as SurfError, Response as SurfResponse};

use super::AsserhttpStatus;

impl AsserhttpStatus<SurfResponse> for SurfResponse {
    fn expect_status_eq(&mut self, status: u16) -> &mut Self {
        assert_eq!(u16::from(self.status()), status);
        self
    }
}

impl AsserhttpStatus<SurfResponse> for Result<SurfResponse, SurfError> {
    fn expect_status_eq(&mut self, status: u16) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_status_eq(status)
    }
}