use std::{fmt::Debug, panic::panic_any};

use actix_http::{
    body::{Body as ActixBody, ResponseBody as ActixResponseBody},
    Error as ActixError,
    Response as ActixResponse,
};
use actix_web::dev::ServiceResponse as ActixServiceResponse;
use serde::{Serialize, de::DeserializeOwned};

use super::super::{
    AsserhttpBody,
    asserter::body::{
        EMPTY_BODY_BYTES_MSG,
        EMPTY_BODY_JSON_MSG,
        EMPTY_BODY_TEXT_MSG,
        INVALID_UTF8_BODY_TEXT_MSG,
        EXPECTED_BODY_ABSENT_MSG,
        EXPECTED_BODY_PRESENT_MSG,
    },
};

impl AsserhttpBody<ActixResponse<ActixBody>> for ActixResponse<ActixBody> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        if let ActixResponseBody::Body(ActixBody::Bytes(actual)) = self.body() {
            serde_json::from_slice(actual.as_ref()).ok()
                .map(|actual| asserter(actual))
                .unwrap_or_else(|| panic_any(EMPTY_BODY_JSON_MSG));
        } else { panic_any(EMPTY_BODY_JSON_MSG) }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        if let ActixResponseBody::Body(ActixBody::Bytes(actual)) = self.body() {
            if let Ok(actual) = String::from_utf8(actual.to_vec()).map_err(anyhow::Error::msg) {
                if !actual.is_empty() {
                    asserter(actual)
                } else { panic_any(EMPTY_BODY_TEXT_MSG) }
            } else { panic_any(INVALID_UTF8_BODY_TEXT_MSG) }
        } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        self
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(&[u8]) {
        if let ActixResponseBody::Body(ActixBody::Bytes(actual)) = self.body() {
            if !actual.is_empty() {
                asserter(actual.as_ref())
            } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        self
    }

    fn expect_body_present(&mut self) -> &mut Self {
        if let ActixResponseBody::Body(ActixBody::Bytes(actual)) = self.body() {
            assert!(!actual.is_empty(), "{}", EXPECTED_BODY_PRESENT_MSG);
        } else { panic_any(EXPECTED_BODY_PRESENT_MSG) }
        self
    }

    fn expect_body_absent(&mut self) -> &mut Self {
        if let ActixResponseBody::Body(ActixBody::Bytes(actual)) = self.body() {
            assert!(actual.is_empty(), "{}", EXPECTED_BODY_ABSENT_MSG);
        }
        self
    }
}

impl AsserhttpBody<ActixResponse<ActixBody>> for Result<ActixResponse<ActixBody>, ActixError> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut ActixResponse<ActixBody>
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut ActixResponse<ActixBody> where F: FnOnce(String) {
        self.as_mut().unwrap().expect_body_text(asserter)
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut ActixResponse<ActixBody> where F: FnOnce(&[u8]) {
        self.as_mut().unwrap().expect_body_bytes(asserter)
    }

    fn expect_body_present(&mut self) -> &mut ActixResponse<ActixBody> {
        self.as_mut().unwrap().expect_body_present()
    }

    fn expect_body_absent(&mut self) -> &mut ActixResponse<ActixBody> {
        self.as_mut().unwrap().expect_body_absent()
    }
}

impl AsserhttpBody<ActixServiceResponse<ActixBody>> for ActixServiceResponse<ActixBody> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        if let ActixResponseBody::Body(ActixBody::Bytes(actual)) = self.response().body() {
            serde_json::from_slice(actual.as_ref()).ok()
                .map(|actual| asserter(actual))
                .unwrap_or_else(|| panic_any(EMPTY_BODY_JSON_MSG));
        } else { panic_any(EMPTY_BODY_JSON_MSG) }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        if let ActixResponseBody::Body(ActixBody::Bytes(actual)) = self.response().body() {
            if let Ok(actual) = String::from_utf8(actual.to_vec()).map_err(anyhow::Error::msg) {
                if !actual.is_empty() {
                    asserter(actual)
                } else { panic_any(EMPTY_BODY_TEXT_MSG) }
            } else { panic_any(INVALID_UTF8_BODY_TEXT_MSG) }
        } else { panic_any(EMPTY_BODY_TEXT_MSG) }
        self
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(&[u8]) {
        if let ActixResponseBody::Body(ActixBody::Bytes(actual)) = self.response().body() {
            if !actual.is_empty() {
                asserter(actual.as_ref())
            } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        } else { panic_any(EMPTY_BODY_BYTES_MSG) }
        self
    }

    fn expect_body_present(&mut self) -> &mut Self {
        if let ActixResponseBody::Body(ActixBody::Bytes(actual)) = self.response().body() {
            assert!(!actual.is_empty(), "{}", EXPECTED_BODY_PRESENT_MSG);
        } else { panic_any(EXPECTED_BODY_PRESENT_MSG) }
        self
    }

    fn expect_body_absent(&mut self) -> &mut Self {
        if let ActixResponseBody::Body(ActixBody::Bytes(actual)) = self.response().body() {
            assert!(actual.is_empty(), "{}", EXPECTED_BODY_ABSENT_MSG);
        }
        self
    }
}