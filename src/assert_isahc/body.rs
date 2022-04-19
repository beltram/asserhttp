use std::{fmt::Debug, panic::panic_any};

use futures_lite::future::block_on;
use isahc::{AsyncReadResponseExt, ReadResponseExt};
use serde::{de::DeserializeOwned, Serialize};

use super::{
    AsyncIsahcResponse,
    IsahcResponse,
    ResultAsyncIsahcResponse,
    ResultIsahcResponse,
    super::{
        AsserhttpBody,
        asserter::body::{
            EMPTY_BODY_BYTES_MSG,
            EMPTY_BODY_JSON_MSG,
            EMPTY_BODY_TEXT_MSG,
            EXPECTED_BODY_ABSENT_MSG,
            EXPECTED_BODY_PRESENT_MSG,
        },
    }
};

impl AsserhttpBody<IsahcResponse> for IsahcResponse {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        if let Ok(actual) = self.json::<B>().map_err(anyhow::Error::msg) {
            asserter(actual)
        } else { panic_any(EMPTY_BODY_JSON_MSG) }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        if let Ok(actual) = self.text().map_err(anyhow::Error::msg) {
            if !actual.is_empty() {
                asserter(actual)
            } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        self
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(&[u8]) {
        let mut actual = vec![];
        self.copy_to(&mut actual).unwrap();
        if !actual.is_empty() {
            asserter(actual.as_slice())
        } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        self
    }

    fn expect_body_present(&mut self) -> &mut Self {
        let mut actual = vec![];
        self.copy_to(&mut actual).unwrap();
        assert!(!actual.is_empty(), "{}", EXPECTED_BODY_PRESENT_MSG);
        self
    }

    fn expect_body_absent(&mut self) -> &mut Self {
        let mut actual = vec![];
        self.copy_to(&mut actual).unwrap();
        assert!(actual.is_empty(), "{}", EXPECTED_BODY_ABSENT_MSG);
        self
    }
}

impl AsserhttpBody<AsyncIsahcResponse> for AsyncIsahcResponse {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        if let Ok(actual) = block_on(self.json::<B>()).map_err(anyhow::Error::msg) {
            asserter(actual)
        } else { panic_any(EMPTY_BODY_JSON_MSG) }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        if let Ok(actual) = block_on(self.text()).map_err(anyhow::Error::msg) {
            if !actual.is_empty() {
                asserter(actual)
            } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        self
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(&[u8]) {
        let mut actual = vec![];
        block_on(self.copy_to(&mut actual)).unwrap();
        if !actual.is_empty() {
            asserter(actual.as_slice())
        } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        self
    }

    fn expect_body_present(&mut self) -> &mut Self {
        let mut actual = vec![];
        block_on(self.copy_to(&mut actual)).unwrap();
        assert!(!actual.is_empty(), "{}", EXPECTED_BODY_PRESENT_MSG);
        self
    }

    fn expect_body_absent(&mut self) -> &mut Self {
        let mut actual = vec![];
        block_on(self.copy_to(&mut actual)).unwrap();
        assert!(actual.is_empty(), "{}", EXPECTED_BODY_ABSENT_MSG);
        self
    }
}

impl AsserhttpBody<IsahcResponse> for ResultIsahcResponse {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut IsahcResponse
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut IsahcResponse where F: FnOnce(String) {
        self.as_mut().unwrap().expect_body_text(asserter)
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut IsahcResponse where F: FnOnce(&[u8]) {
        self.as_mut().unwrap().expect_body_bytes(asserter)
    }

    fn expect_body_present(&mut self) -> &mut IsahcResponse {
        self.as_mut().unwrap().expect_body_present()
    }

    fn expect_body_absent(&mut self) -> &mut IsahcResponse {
        self.as_mut().unwrap().expect_body_absent()
    }
}

impl AsserhttpBody<AsyncIsahcResponse> for ResultAsyncIsahcResponse {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut AsyncIsahcResponse
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut AsyncIsahcResponse where F: FnOnce(String) {
        self.as_mut().unwrap().expect_body_text(asserter)
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut AsyncIsahcResponse where F: FnOnce(&[u8]) {
        self.as_mut().unwrap().expect_body_bytes(asserter)
    }

    fn expect_body_present(&mut self) -> &mut AsyncIsahcResponse {
        self.as_mut().unwrap().expect_body_present()
    }

    fn expect_body_absent(&mut self) -> &mut AsyncIsahcResponse {
        self.as_mut().unwrap().expect_body_absent()
    }
}