use std::{fmt::Debug, io::Read, panic::panic_any, str::from_utf8};

use async_std::task::block_on;
use rocket::{
    local::{
        asynchronous::LocalResponse as AsyncRocketResponse,
        blocking::LocalResponse as BlockingRocketResponse,
    },
    tokio::io::AsyncReadExt,
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

impl<'a> AsserhttpBody<BlockingRocketResponse<'a>> for BlockingRocketResponse<'a> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        let mut buf: Vec<u8> = vec![];
        self.read_to_end(&mut buf).unwrap_or_default();
        if let Ok(actual) = serde_json::from_slice::<B>(&buf) {
            asserter(actual)
        } else { panic_any(EMPTY_BODY_JSON_MSG) }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        let mut buf: Vec<u8> = vec![];
        self.read_to_end(&mut buf).unwrap_or_default();
        if let Ok(actual) = from_utf8(&buf) {
            if !actual.is_empty() {
                asserter(actual.to_string())
            } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        } else { panic_any(INVALID_UTF8_BODY_TEXT_MSG) }
        self
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(&[u8]) {
        let mut buf: Vec<u8> = vec![];
        self.read_to_end(&mut buf).unwrap_or_default();
        if !buf.is_empty() {
            asserter(buf.as_slice())
        } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        self
    }

    fn expect_body_present(&mut self) -> &mut Self {
        let mut buf: Vec<u8> = vec![];
        self.read_to_end(&mut buf).unwrap_or_default();
        assert!(!buf.is_empty(), "{}", EXPECTED_BODY_PRESENT_MSG);
        self
    }

    fn expect_body_absent(&mut self) -> &mut Self {
        let mut buf: Vec<u8> = vec![];
        self.read_to_end(&mut buf).unwrap_or_default();
        assert!(buf.is_empty(), "{}", EXPECTED_BODY_ABSENT_MSG);
        self
    }
}

impl<'a> AsserhttpBody<AsyncRocketResponse<'a>> for AsyncRocketResponse<'a> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        let mut buf: Vec<u8> = vec![];
        block_on(self.read_to_end(&mut buf)).unwrap_or_default();
        if let Ok(actual) = serde_json::from_slice::<B>(&buf) {
            asserter(actual)
        } else { panic_any(EMPTY_BODY_JSON_MSG) }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        let mut buf: Vec<u8> = vec![];
        block_on(self.read_to_end(&mut buf)).unwrap_or_default();
        if let Ok(actual) = from_utf8(&buf) {
            if !actual.is_empty() {
                asserter(actual.to_string())
            } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        self
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(&[u8]) {
        let mut buf: Vec<u8> = vec![];
        block_on(self.read_to_end(&mut buf)).unwrap_or_default();
        if !buf.is_empty() {
            asserter(buf.as_slice())
        } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        self
    }

    fn expect_body_present(&mut self) -> &mut Self {
        let mut buf: Vec<u8> = vec![];
        block_on(self.read_to_end(&mut buf)).unwrap_or_default();
        assert!(!buf.is_empty(), "{}", EXPECTED_BODY_PRESENT_MSG);
        self
    }

    fn expect_body_absent(&mut self) -> &mut Self {
        let mut buf: Vec<u8> = vec![];
        block_on(self.read_to_end(&mut buf)).unwrap_or_default();
        assert!(buf.is_empty(), "{}", EXPECTED_BODY_ABSENT_MSG);
        self
    }
}