use std::{fmt::Debug, panic::panic_any};

use async_std::task::block_on;
use awc::error::SendRequestError as AwcError;
use serde::{de::DeserializeOwned, Serialize};

use super::{
    AwcResponse,
    super::{
        AsserhttpBody,
        asserter::body::{
            EMPTY_BODY_BYTES_MSG,
            EMPTY_BODY_JSON_MSG,
            EMPTY_BODY_TEXT_MSG,
            EXPECTED_BODY_ABSENT_MSG,
            EXPECTED_BODY_PRESENT_MSG,
        },
    },
};

impl AsserhttpBody<AwcResponse> for AwcResponse {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        block_on(self.body()).ok()
            .and_then(|b| serde_json::from_slice::<B>(&b).ok())
            .map(|actual| asserter(actual))
            .unwrap_or_else(|| panic_any(EMPTY_BODY_JSON_MSG));
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        block_on(self.body()).ok()
            .and_then(|b| String::from_utf8(b.to_vec()).ok())
            .filter(|it| !it.is_empty())
            .map(|actual| asserter(actual))
            .unwrap_or_else(|| panic_any(EMPTY_BODY_TEXT_MSG));
        self
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(&[u8]) {
        block_on(self.body()).ok()
            .filter(|it| !it.is_empty())
            .map(|actual| asserter(&actual))
            .unwrap_or_else(|| panic_any(EMPTY_BODY_BYTES_MSG));
        self
    }

    fn expect_body_present(&mut self) -> &mut Self {
        let actual = block_on(self.body()).unwrap_or_default();
        assert!(!actual.is_empty(), "{}", EXPECTED_BODY_PRESENT_MSG);
        self
    }

    fn expect_body_absent(&mut self) -> &mut Self {
        let actual = block_on(self.body()).unwrap_or_default();
        assert!(actual.is_empty(), "{}", EXPECTED_BODY_ABSENT_MSG);
        self
    }
}

impl AsserhttpBody<AwcResponse> for Result<AwcResponse, AwcError> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut AwcResponse
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut AwcResponse where F: FnOnce(String) {
        self.as_mut().unwrap().expect_body_text(asserter)
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut AwcResponse where F: FnOnce(&[u8]) {
        self.as_mut().unwrap().expect_body_bytes(asserter)
    }

    fn expect_body_present(&mut self) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_body_present()
    }

    fn expect_body_absent(&mut self) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_body_absent()
    }
}
