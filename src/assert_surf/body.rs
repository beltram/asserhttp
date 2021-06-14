use std::fmt::Debug;

use async_std::task::block_on;
use serde::de::DeserializeOwned;
use surf::{Error as SurfError, Response as SurfResponse};

use super::super::AsserhttpBody;

impl AsserhttpBody<SurfResponse> for SurfResponse {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        if let Ok(actual) = block_on(self.body_json::<B>()).map_err(anyhow::Error::msg) {
            asserter(actual)
        } else { panic!("expected a json body but no response body was present") }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        if let Ok(actual) = block_on(self.body_string()).map_err(anyhow::Error::msg) {
            if !actual.is_empty() {
                asserter(actual)
            } else { panic!("expected a text body but no response body was present") }
        } else { panic!("expected a text body but no response body was present") }
        self
    }
}

impl AsserhttpBody<SurfResponse> for Result<SurfResponse, SurfError> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut SurfResponse
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut SurfResponse where F: FnOnce(String) {
        self.as_mut().unwrap().expect_body_text(asserter)
    }
}
