use surf::{Error as SurfError, http::headers::HeaderName as SurfHeaderName, http::headers::HeaderValue as SurfHeaderValue, Response as SurfResponse};

use super::super::{
    AsserhttpHeader,
    asserter::header::{assert_header_key, assert_header_key_absent, assert_header_value, assert_header_values},
};

impl AsserhttpHeader<SurfResponse> for SurfResponse {
    fn expect_header(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut Self {
        assert_header_key(self.header_names().map(SurfHeaderName::as_str), key.as_ref());
        let values = self.header(key.as_ref()).unwrap().into_iter()
            .map(SurfHeaderValue::as_str)
            .flat_map(|v: &str| v.split(',').map(|s| s.trim()));
        assert_header_value(values, key.as_ref(), value.as_ref());
        self
    }

    fn expect_headers<'a>(&mut self, key: impl AsRef<str>, value: impl Into<Vec<&'a str>>) -> &mut Self {
        assert_header_key(self.header_names().map(SurfHeaderName::as_str), key.as_ref());
        let values = self.header(key.as_ref()).unwrap().into_iter()
            .map(SurfHeaderValue::as_str)
            .flat_map(|v: &str| v.split(',').map(|s| s.trim()));
        assert_header_values(values, key.as_ref(), value.into());
        self
    }

    fn expect_header_present(&mut self, key: impl AsRef<str>) -> &mut Self {
        assert_header_key(self.header_names().map(SurfHeaderName::as_str), key.as_ref());
        self
    }

    fn expect_header_absent(&mut self, key: impl AsRef<str>) -> &mut Self {
        assert_header_key_absent(self.header_names().map(SurfHeaderName::as_str), key.as_ref());
        self
    }
}

impl AsserhttpHeader<SurfResponse> for Result<SurfResponse, SurfError> {
    fn expect_header(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_header(key, value)
    }

    fn expect_headers<'a>(&mut self, key: impl AsRef<str>, value: impl Into<Vec<&'a str>>) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_headers(key, value)
    }

    fn expect_header_present(&mut self, key: impl AsRef<str>) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_header_present(key)
    }
    fn expect_header_absent(&mut self, key: impl AsRef<str>) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_header_absent(key)
    }
}