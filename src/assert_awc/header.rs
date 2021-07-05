use actix_http::{encoding::Decoder, Payload};
use awc::error::SendRequestError as AwcError;

use super::super::{
    AsserhttpHeader,
    asserter::header::{assert_header_key, assert_header_key_absent, assert_header_value, assert_header_values},
};

pub type AwcResponse = awc::ClientResponse<Decoder<Payload>>;

impl AsserhttpHeader<AwcResponse> for AwcResponse {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut Self {
        let key = key.into();
        assert_header_key(self.headers().iter().map(|(name, _)| name.as_str()), key);
        let values = self.headers().get(key)
            .and_then(|it| it.to_str().ok())
            .map(|v: &str| v.split(',').map(|s| s.trim()))
            .unwrap();
        assert_header_value(values, key, value.into());
        self
    }

    fn expect_headers<'a, K: Into<&'a str>, V: Into<Vec<&'a str>>>(&mut self, key: K, value: V) -> &mut Self {
        let key = key.into();
        assert_header_key(self.headers().iter().map(|(name, _)| name.as_str()), key);
        let values = self.headers().get(key)
            .and_then(|it| it.to_str().ok())
            .map(|v: &str| v.split(',').map(|s| s.trim()))
            .unwrap();
        assert_header_values(values, key, value.into());
        self
    }

    fn expect_header_present<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut Self {
        assert_header_key(self.headers().iter().map(|(name, _)| name.as_str()), key.into());
        self
    }

    fn expect_header_absent<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut Self {
        assert_header_key_absent(self.headers().iter().map(|(name, _)| name.as_str()), key.into());
        self
    }
}

impl AsserhttpHeader<AwcResponse> for Result<AwcResponse, AwcError> {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_header(key, value)
    }

    fn expect_headers<'a, K: Into<&'a str>, V: Into<Vec<&'a str>>>(&mut self, key: K, value: V) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_headers(key, value)
    }

    fn expect_header_present<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_header_present(key)
    }
    fn expect_header_absent<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_header_absent(key)
    }
}