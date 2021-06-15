use std::{fmt::Debug, panic::panic_any};

use async_std::task::block_on;
use serde::de::DeserializeOwned;
use surf::{Error as SurfError, Response as SurfResponse};

use super::super::{
    AsserhttpBody,
    asserter::body::{
        EMPTY_BODY_BYTES_MSG,
        EMPTY_BODY_JSON_MSG,
        EMPTY_BODY_TEXT_MSG,
        EXPECTED_BODY_ABSENT_MSG,
        EXPECTED_BODY_PRESENT_MSG,
    },
};

impl AsserhttpBody<SurfResponse> for SurfResponse {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        if let Ok(actual) = block_on(self.body_json::<B>()).map_err(anyhow::Error::msg) {
            asserter(actual)
        } else { panic_any(EMPTY_BODY_JSON_MSG) }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        if let Ok(actual) = block_on(self.body_string()).map_err(anyhow::Error::msg) {
            if !actual.is_empty() {
                asserter(actual)
            } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        self
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(&[u8]) {
        if let Ok(actual) = block_on(self.body_bytes()).map_err(anyhow::Error::msg) {
            if !actual.is_empty() {
                asserter(actual.as_slice())
            } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        self
    }

    fn expect_body_present(&mut self) -> &mut Self {
        if let Ok(actual) = block_on(self.body_bytes()).map_err(anyhow::Error::msg) {
            assert!(!actual.is_empty(), "{}", EXPECTED_BODY_PRESENT_MSG);
        } else { panic_any(EXPECTED_BODY_PRESENT_MSG) }
        self
    }

    fn expect_body_absent(&mut self) -> &mut Self {
        if let Ok(actual) = block_on(self.body_bytes()).map_err(anyhow::Error::msg) {
            assert!(actual.is_empty(), "{}", EXPECTED_BODY_ABSENT_MSG);
        }
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

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut SurfResponse where F: FnOnce(&[u8]) {
        self.as_mut().unwrap().expect_body_bytes(asserter)
    }

    fn expect_body_present(&mut self) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_body_present()
    }

    fn expect_body_absent(&mut self) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_body_absent()
    }
}
