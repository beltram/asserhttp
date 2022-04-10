use std::{fmt::Debug, panic::panic_any};

use futures_lite::future::block_on;
use hyper::{
    Body as HyperBody,
    body::HttpBody,
    Response as HyperResponse,
    Result as HyperResult,
};
use serde::{de::DeserializeOwned, Serialize};

use super::super::{
    AsserhttpBody,
    asserter::body::{
        EMPTY_BODY_BYTES_MSG,
        EMPTY_BODY_JSON_MSG,
        EMPTY_BODY_TEXT_MSG,
        EXPECTED_BODY_ABSENT_MSG,
        EXPECTED_BODY_PRESENT_MSG,
        INVALID_UTF8_BODY_TEXT_MSG,
    },
};

impl AsserhttpBody<HyperResponse<HyperBody>> for HyperResponse<HyperBody> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        let mut actual: Vec<u8> = vec![];
        while let Some(Ok(chunk)) = block_on(self.body_mut().data()) {
            chunk.into_iter().for_each(|b| actual.push(b));
        }
        if let Ok(actual) = serde_json::from_slice::<B>(&actual).map_err(anyhow::Error::msg) {
            asserter(actual)
        } else { panic_any(EMPTY_BODY_JSON_MSG) }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        let mut actual: Vec<u8> = vec![];
        while let Some(Ok(chunk)) = block_on(self.body_mut().data()) {
            chunk.into_iter().for_each(|b| actual.push(b));
        }
        if let Ok(actual) = String::from_utf8(actual).map_err(anyhow::Error::msg) {
            if !actual.is_empty() {
                asserter(actual)
            } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        } else { panic_any(INVALID_UTF8_BODY_TEXT_MSG) }
        self
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(&[u8]) {
        let mut actual: Vec<u8> = vec![];
        while let Some(Ok(chunk)) = block_on(self.body_mut().data()) {
            chunk.into_iter().for_each(|b| actual.push(b));
        }
        if !actual.is_empty() {
            asserter(actual.as_slice())
        } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        self
    }

    fn expect_body_present(&mut self) -> &mut Self {
        let mut actual: Vec<u8> = vec![];
        while let Some(Ok(chunk)) = block_on(self.body_mut().data()) {
            chunk.into_iter().for_each(|b| actual.push(b));
        }
        assert!(!actual.is_empty(), "{}", EXPECTED_BODY_PRESENT_MSG);
        self
    }

    fn expect_body_absent(&mut self) -> &mut Self {
        let mut actual: Vec<u8> = vec![];
        while let Some(Ok(chunk)) = block_on(self.body_mut().data()) {
            chunk.into_iter().for_each(|b| actual.push(b));
        }
        assert!(actual.is_empty(), "{}", EXPECTED_BODY_ABSENT_MSG);
        self
    }
}

impl AsserhttpBody<HyperResponse<HyperBody>> for HyperResult<HyperResponse<HyperBody>> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut HyperResponse<HyperBody>
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut HyperResponse<HyperBody> where F: FnOnce(String) {
        self.as_mut().unwrap().expect_body_text(asserter)
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut HyperResponse<HyperBody> where F: FnOnce(&[u8]) {
        self.as_mut().unwrap().expect_body_bytes(asserter)
    }

    fn expect_body_present(&mut self) -> &mut HyperResponse<HyperBody> {
        self.as_mut().unwrap().expect_body_present()
    }

    fn expect_body_absent(&mut self) -> &mut HyperResponse<HyperBody> {
        self.as_mut().unwrap().expect_body_absent()
    }
}
