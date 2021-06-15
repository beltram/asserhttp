use surf::{
    Error as SurfError,
    http::headers::HeaderName as SurfHeaderName,
    http::headers::HeaderValue as SurfHeaderValue,
    Response as SurfResponse,
};

use super::super::{
    AsserhttpHeader,
    asserter::header::{assert_header_key, assert_header_key_absent, assert_header_value},
};

impl AsserhttpHeader<SurfResponse> for SurfResponse {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut Self {
        let key = key.into();
        assert_header_key(self.header_names().map(SurfHeaderName::as_str), key);
        let values = self.header(key).unwrap().into_iter()
            .map(SurfHeaderValue::as_str)
            .flat_map(|v: &str| v.split(',').map(|s| s.trim()));
        assert_header_value(values, key, value.into());
        self
    }

    fn expect_header_present<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut Self {
        assert_header_key(self.header_names().map(SurfHeaderName::as_str), key.into());
        self
    }

    fn expect_header_absent<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut Self {
        assert_header_key_absent(self.header_names().map(SurfHeaderName::as_str), key.into());
        self
    }
}

impl AsserhttpHeader<SurfResponse> for Result<SurfResponse, SurfError> {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_header(key, value)
    }

    fn expect_header_present<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_header_present(key)
    }
    fn expect_header_absent<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut SurfResponse {
        self.as_mut().unwrap().expect_header_absent(key)
    }
}