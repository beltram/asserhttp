use std::{fmt::Debug, panic::panic_any};

use async_std::task::block_on;
use isahc::{AsyncBody as IsahcAsyncBody, AsyncReadResponseExt, Body as IsahcBody, Error as IsahcError, ReadResponseExt, Response as IsahcResponse};
use serde::de::DeserializeOwned;

use super::super::{
    AsserhttpBody,
    asserter::body::{EMPTY_BODY_BYTES_MSG, EMPTY_BODY_JSON_MSG, EMPTY_BODY_TEXT_MSG},
};

impl AsserhttpBody<IsahcResponse<IsahcBody>> for IsahcResponse<IsahcBody> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
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
}

impl AsserhttpBody<IsahcResponse<IsahcAsyncBody>> for IsahcResponse<IsahcAsyncBody> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
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
}

impl AsserhttpBody<IsahcResponse<IsahcBody>> for Result<IsahcResponse<IsahcBody>, IsahcError> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut IsahcResponse<IsahcBody>
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut IsahcResponse<IsahcBody> where F: FnOnce(String) {
        self.as_mut().unwrap().expect_body_text(asserter)
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut IsahcResponse<IsahcBody> where F: FnOnce(&[u8]) {
        self.as_mut().unwrap().expect_body_bytes(asserter)
    }
}

impl AsserhttpBody<IsahcResponse<IsahcAsyncBody>> for Result<IsahcResponse<IsahcAsyncBody>, IsahcError> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut IsahcResponse<IsahcAsyncBody>
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut IsahcResponse<IsahcAsyncBody> where F: FnOnce(String) {
        self.as_mut().unwrap().expect_body_text(asserter)
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut IsahcResponse<IsahcAsyncBody> where F: FnOnce(&[u8]) {
        self.as_mut().unwrap().expect_body_bytes(asserter)
    }
}