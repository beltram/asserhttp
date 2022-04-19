use std::{fmt::Debug, panic::panic_any};

use actix_http::body::MessageBody;
use serde::{de::DeserializeOwned, Serialize};

use super::{
    ActixResponse, ActixServiceResponse, ResultActixResponse,
    super::{
        AsserhttpBody,
        asserter::body::{
            EMPTY_BODY_BYTES_MSG,
            EMPTY_BODY_JSON_MSG,
            EMPTY_BODY_TEXT_MSG,
            EXPECTED_BODY_ABSENT_MSG,
            EXPECTED_BODY_PRESENT_MSG,
            INVALID_UTF8_BODY_TEXT_MSG,
        },
    }};

impl AsserhttpBody<ActixResponse> for ActixResponse {
    fn expect_body_json<Body, F>(&mut self, asserter: F) -> &mut Self
        where Body: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(Body) {
        if let Some(actual) = body_bytes(self) {
            serde_json::from_slice(actual.as_ref()).ok()
                .map(asserter)
                .unwrap_or_else(|| panic_any(EMPTY_BODY_JSON_MSG));
        } else { panic_any(EMPTY_BODY_JSON_MSG) }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        if let Some(actual) = body_bytes(self) {
            if let Ok(actual) = String::from_utf8(actual.to_vec()).map_err(anyhow::Error::msg) {
                if !actual.is_empty() {
                    asserter(actual)
                } else { panic_any(EMPTY_BODY_TEXT_MSG) }
            } else { panic_any(INVALID_UTF8_BODY_TEXT_MSG) }
        } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        self
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(&[u8]) {
        if let Some(actual) = body_bytes(self) {
            if !actual.is_empty() {
                asserter(actual.as_ref())
            } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        self
    }

    fn expect_body_present(&mut self) -> &mut Self {
        if let Some(actual) = body_bytes(self) {
            assert!(!actual.is_empty(), "{}", EXPECTED_BODY_PRESENT_MSG);
        } else { panic_any(EXPECTED_BODY_PRESENT_MSG) }
        self
    }

    fn expect_body_absent(&mut self) -> &mut Self {
        if let Some(actual) = body_bytes(self) {
            assert!(actual.is_empty(), "{}", EXPECTED_BODY_ABSENT_MSG);
        }
        self
    }
}

impl AsserhttpBody<ActixResponse> for ResultActixResponse {
    fn expect_body_json<Body, F>(&mut self, asserter: F) -> &mut ActixResponse
        where Body: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(Body) {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut ActixResponse where F: FnOnce(String) {
        self.as_mut().unwrap().expect_body_text(asserter)
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut ActixResponse where F: FnOnce(&[u8]) {
        self.as_mut().unwrap().expect_body_bytes(asserter)
    }

    fn expect_body_present(&mut self) -> &mut ActixResponse {
        self.as_mut().unwrap().expect_body_present()
    }

    fn expect_body_absent(&mut self) -> &mut ActixResponse {
        self.as_mut().unwrap().expect_body_absent()
    }
}

impl AsserhttpBody<ActixServiceResponse> for ActixServiceResponse {
    fn expect_body_json<Body, F>(&mut self, asserter: F) -> &mut Self
        where Body: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(Body) {
        if let Some(actual) = body_bytes(self.response_mut()) {
            serde_json::from_slice(actual.as_ref()).ok()
                .map(asserter)
                .unwrap_or_else(|| panic_any(EMPTY_BODY_JSON_MSG));
        } else { panic_any(EMPTY_BODY_JSON_MSG) }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        if let Some(actual) = body_bytes(self.response_mut()) {
            if let Ok(actual) = String::from_utf8(actual.to_vec()).map_err(anyhow::Error::msg) {
                if !actual.is_empty() {
                    asserter(actual)
                } else { panic_any(EMPTY_BODY_TEXT_MSG) }
            } else { panic_any(INVALID_UTF8_BODY_TEXT_MSG) }
        } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        self
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(&[u8]) {
        if let Some(actual) = body_bytes(self.response_mut()) {
            if !actual.is_empty() {
                asserter(actual.as_ref())
            } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        self
    }

    fn expect_body_present(&mut self) -> &mut Self {
        if let Some(actual) = body_bytes(self.response_mut()) {
            assert!(!actual.is_empty(), "{}", EXPECTED_BODY_PRESENT_MSG);
        } else { panic_any(EXPECTED_BODY_PRESENT_MSG) }
        self
    }

    fn expect_body_absent(&mut self) -> &mut Self {
        if let Some(actual) = body_bytes(self.response_mut()) {
            assert!(actual.is_empty(), "{}", EXPECTED_BODY_ABSENT_MSG);
        }
        self
    }
}

fn body_bytes(original: &mut ActixResponse) -> Option<actix_web::web::Bytes> {
    let mut resp_cpy = ActixResponse::build(original.status());
    original.headers().iter().for_each(|h| { resp_cpy.insert_header(h); });
    let mut resp_cpy = resp_cpy.finish();
    std::mem::swap(original, &mut resp_cpy);
    let (_, body) = resp_cpy.into_parts();
    body.try_into_bytes().ok()
}