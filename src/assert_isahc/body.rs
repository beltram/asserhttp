use std::fmt::Debug;

use async_std::task::block_on;
use isahc::{AsyncBody as IsahcAsyncBody, AsyncReadResponseExt, Body as IsahcBody, Error as IsahcError, ReadResponseExt, Response as IsahcResponse};
use serde::de::DeserializeOwned;

use super::super::AsserhttpBody;

impl AsserhttpBody<IsahcResponse<IsahcBody>> for IsahcResponse<IsahcBody> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        if let Ok(actual) = self.json::<B>().map_err(anyhow::Error::msg) {
            asserter(actual)
        } else { panic!("expected a json body but no response body was present") }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        if let Ok(actual) = self.text().map_err(anyhow::Error::msg) {
            if !actual.is_empty() {
                asserter(actual)
            } else { panic!("expected a text body but no response body was present") }
        } else { panic!("expected a text body but no response body was present") }
        self
    }
}

impl AsserhttpBody<IsahcResponse<IsahcAsyncBody>> for IsahcResponse<IsahcAsyncBody> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        if let Ok(actual) = block_on(self.json::<B>()).map_err(anyhow::Error::msg) {
            asserter(actual)
        } else { panic!("expected a json body but no response body was present") }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut Self where F: FnOnce(String) {
        if let Ok(actual) = block_on(self.text()).map_err(anyhow::Error::msg) {
            if !actual.is_empty() {
                asserter(actual)
            } else { panic!("expected a text body but no response body was present") }
        } else { panic!("expected a text body but no response body was present") }
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
}