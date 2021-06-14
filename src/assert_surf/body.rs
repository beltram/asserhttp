use std::fmt::Debug;

use async_std::task::block_on;
use serde::de::DeserializeOwned;
use surf::{Error as SurfError, Response as SurfResponse};

use super::super::{
    AsserhttpBody,
    asserter::body::assert_text_body,
};

impl AsserhttpBody<SurfResponse> for SurfResponse {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        if let Ok(actual) = block_on(self.body_json::<B>()).map_err(anyhow::Error::msg) {
            asserter(actual)
        } else { panic!("expected a json body but no response body was present") }
        self
    }

    fn expect_body_text<B>(&mut self, body: B) -> &mut Self where B: Into<String> {
        let actual = block_on(self.body_string()).map_err(anyhow::Error::msg);
        assert_text_body(actual, body.into());
        self
    }
}

impl AsserhttpBody<SurfResponse> for Result<SurfResponse, SurfError> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut SurfResponse
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<B>(&mut self, body: B) -> &mut SurfResponse where B: Into<String> {
        self.as_mut().unwrap().expect_body_text(body)
    }
}
