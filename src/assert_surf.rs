use surf::{Error as SurfError, Response as SurfResponse};

use super::{Asserhttp, TryAsserhttp};

impl Asserhttp for SurfResponse {
    fn assert_status_eq(&mut self, status: u16) -> &mut Self {
        assert_eq!(u16::from(self.status()), status);
        self
    }
}

impl TryAsserhttp<SurfResponse> for Result<SurfResponse, SurfError> {
    fn assert_status_eq(&mut self, status: u16) -> &mut SurfResponse {
        self.as_mut().unwrap().assert_status_eq(status)
    }
}