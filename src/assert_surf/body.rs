use std::fmt::Debug;

use async_std::task::block_on;
use serde::de::DeserializeOwned;
use surf::{
    Error as SurfError,
    Response as SurfResponse,
};

use super::super::{AsserhttpBody, asserter::body::assert_json_body};

impl AsserhttpBody<SurfResponse> for SurfResponse {
    fn expect_body_json<B>(&mut self, body: B) -> &mut Self where B: DeserializeOwned + PartialEq + Debug + Unpin {
        let actual = block_on(self.body_json::<B>()).map_err(anyhow::Error::msg);
        assert_json_body(actual, body);
        self
    }
}

impl AsserhttpBody<SurfResponse> for Result<SurfResponse, SurfError> {
    fn expect_body_json<B>(&mut self, body: B) -> &mut SurfResponse where B: DeserializeOwned + PartialEq + Debug + Unpin {
        self.as_mut().unwrap().expect_body_json(body)
    }
}
