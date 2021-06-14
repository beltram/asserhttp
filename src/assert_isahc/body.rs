use std::fmt::Debug;

use async_std::task::block_on;
use isahc::{
    AsyncBody as IsahcAsyncBody,
    AsyncReadResponseExt,
    Body as IsahcBody,
    Error as IsahcError,
    ReadResponseExt,
    Response as IsahcResponse,
};
use serde::de::DeserializeOwned;

use super::super::{
    AsserhttpBody,
    asserter::body::{assert_json_body, assert_text_body},
};

impl AsserhttpBody<IsahcResponse<IsahcBody>> for IsahcResponse<IsahcBody> {
    fn expect_body_json<B>(&mut self, body: B) -> &mut Self where B: DeserializeOwned + PartialEq + Debug + Unpin {
        let actual = self.json::<B>().map_err(anyhow::Error::msg);
        assert_json_body(actual, body);
        self
    }

    fn expect_body_text<B>(&mut self, body: B) -> &mut Self where B: Into<String> {
        let actual = self.text().map_err(anyhow::Error::msg);
        assert_text_body(actual, body.into());
        self
    }
}

impl AsserhttpBody<IsahcResponse<IsahcAsyncBody>> for IsahcResponse<IsahcAsyncBody> {
    fn expect_body_json<B>(&mut self, body: B) -> &mut Self where B: DeserializeOwned + PartialEq + Debug + Unpin {
        let actual = block_on(self.json::<B>()).map_err(anyhow::Error::msg);
        assert_json_body(actual, body);
        self
    }

    fn expect_body_text<B>(&mut self, body: B) -> &mut Self where B: Into<String> {
        let actual = block_on(self.text()).map_err(anyhow::Error::msg);
        assert_text_body(actual, body.into());
        self
    }
}

impl AsserhttpBody<IsahcResponse<IsahcBody>> for Result<IsahcResponse<IsahcBody>, IsahcError> {
    fn expect_body_json<B>(&mut self, body: B) -> &mut IsahcResponse<IsahcBody> where B: DeserializeOwned + PartialEq + Debug + Unpin {
        self.as_mut().unwrap().expect_body_json(body)
    }

    fn expect_body_text<B>(&mut self, body: B) -> &mut IsahcResponse<IsahcBody> where B: Into<String> {
        self.as_mut().unwrap().expect_body_text(body)
    }
}

impl AsserhttpBody<IsahcResponse<IsahcAsyncBody>> for Result<IsahcResponse<IsahcAsyncBody>, IsahcError> {
    fn expect_body_json<B>(&mut self, body: B) -> &mut IsahcResponse<IsahcAsyncBody> where B: DeserializeOwned + PartialEq + Debug + Unpin {
        self.as_mut().unwrap().expect_body_json(body)
    }

    fn expect_body_text<B>(&mut self, body: B) -> &mut IsahcResponse<IsahcAsyncBody> where B: Into<String> {
        self.as_mut().unwrap().expect_body_text(body)
    }
}