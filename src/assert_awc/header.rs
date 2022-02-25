use actix_http::{encoding::Decoder, Payload};
use awc::error::SendRequestError as AwcError;

use super::super::{
    AsserhttpHeader,
    asserter::header::{assert_header_key, assert_header_key_absent, assert_header_value, assert_header_values},
};

pub type AwcResponse = awc::ClientResponse<Decoder<Payload>>;

impl AsserhttpHeader<AwcResponse> for AwcResponse {
    fn expect_header<'a>(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut Self {
        assert_header_key(self.headers().iter().map(|(name, _)| name.as_str()), key.as_ref());
        let values = self.headers().get(key.as_ref())
            .and_then(|it| it.to_str().ok())
            .map(|v: &str| v.split(',').map(|s| s.trim()))
            .unwrap();
        assert_header_value(values, key.as_ref(), value.as_ref());
        self
    }

    fn expect_headers<'a, V: Into<Vec<&'a str>>>(&mut self, key: impl AsRef<str>, value: V) -> &mut Self {
        assert_header_key(self.headers().iter().map(|(name, _)| name.as_str()), key.as_ref());
        let values = self.headers().get(key.as_ref())
            .and_then(|it| it.to_str().ok())
            .map(|v: &str| v.split(',').map(|s| s.trim()))
            .unwrap();
        assert_header_values(values, key.as_ref(), value.into());
        self
    }

    fn expect_header_present<'a>(&mut self, key: impl AsRef<str>) -> &mut Self {
        assert_header_key(self.headers().iter().map(|(name, _)| name.as_str()), key.as_ref());
        self
    }

    fn expect_header_absent<'a>(&mut self, key: impl AsRef<str>) -> &mut Self {
        assert_header_key_absent(self.headers().iter().map(|(name, _)| name.as_str()), key.as_ref());
        self
    }
}

impl AsserhttpHeader<AwcResponse> for Result<AwcResponse, AwcError> {
    fn expect_header<'a>(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_header(key, value)
    }

    fn expect_headers<'a, V: Into<Vec<&'a str>>>(&mut self, key: impl AsRef<str>, value: V) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_headers(key, value)
    }

    fn expect_header_present<'a>(&mut self, key: impl AsRef<str>) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_header_present(key)
    }
    fn expect_header_absent<'a>(&mut self, key: impl AsRef<str>) -> &mut AwcResponse {
        self.as_mut().unwrap().expect_header_absent(key)
    }
}