use std::fmt::Debug;

use async_std::task::block_on;
use isahc::{AsyncBody as IsahcAsyncBody, AsyncReadResponseExt, Body as IsahcBody, Error as IsahcError, ReadResponseExt, Response as IsahcResponse};
use serde::de::DeserializeOwned;

use super::super::{
    AsserhttpBody,
    asserter::body::assert_text_body,
};

impl AsserhttpBody<IsahcResponse<IsahcBody>> for IsahcResponse<IsahcBody> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut Self
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        if let Ok(actual) = self.json::<B>().map_err(anyhow::Error::msg) {
            asserter(actual)
        } else { panic!("expected a json body but no response body was present") }
        self
    }

    fn expect_body_text<B>(&mut self, body: B) -> &mut Self where B: Into<String> {
        let actual = self.text().map_err(anyhow::Error::msg);
        assert_text_body(actual, body.into());
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

    fn expect_body_text<B>(&mut self, body: B) -> &mut Self where B: Into<String> {
        let actual = block_on(self.text()).map_err(anyhow::Error::msg);
        assert_text_body(actual, body.into());
        self
    }
}

impl AsserhttpBody<IsahcResponse<IsahcBody>> for Result<IsahcResponse<IsahcBody>, IsahcError> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut IsahcResponse<IsahcBody>
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<B>(&mut self, body: B) -> &mut IsahcResponse<IsahcBody> where B: Into<String> {
        self.as_mut().unwrap().expect_body_text(body)
    }
}

impl AsserhttpBody<IsahcResponse<IsahcAsyncBody>> for Result<IsahcResponse<IsahcAsyncBody>, IsahcError> {
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut IsahcResponse<IsahcAsyncBody>
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B) {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<B>(&mut self, body: B) -> &mut IsahcResponse<IsahcAsyncBody> where B: Into<String> {
        self.as_mut().unwrap().expect_body_text(body)
    }
}