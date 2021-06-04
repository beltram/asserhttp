use isahc::{
    AsyncBody as IsahcAsyncBody,
    AsyncBody, Body as IsahcBody,
    Error as IsahcError,
    http::header::HeaderMap as IsahcHeaderMap,
    http::header::HeaderName as IsahcHeaderName,
    Response as IsahcResponse,
};

use super::super::{
    AsserhttpHeader,
    asserter::header::{assert_header_key, assert_header_value},
};

impl AsserhttpHeader<IsahcResponse<IsahcBody>> for IsahcResponse<IsahcBody> {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut IsahcResponse<IsahcBody> {
        let key = key.into();
        assert_header_key(header_keys(self.headers()), key);
        assert_header_value(header_values(key, self.headers()), key, value.into());
        self
    }
}

impl AsserhttpHeader<IsahcResponse<IsahcAsyncBody>> for IsahcResponse<IsahcAsyncBody> {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut IsahcResponse<AsyncBody> {
        let key = key.into();
        assert_header_key(header_keys(self.headers()), key);
        assert_header_value(header_values(key, self.headers()), key, value.into());
        self
    }
}

impl AsserhttpHeader<IsahcResponse<IsahcBody>> for Result<IsahcResponse<IsahcBody>, IsahcError> {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut IsahcResponse<IsahcBody> {
        self.as_mut().unwrap().expect_header(key, value)
    }
}

impl AsserhttpHeader<IsahcResponse<IsahcAsyncBody>> for Result<IsahcResponse<IsahcAsyncBody>, IsahcError> {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut IsahcResponse<AsyncBody> {
        self.as_mut().unwrap().expect_header(key, value)
    }
}

fn header_keys(headers: &IsahcHeaderMap) -> impl Iterator<Item=&str> {
    headers.keys().map(IsahcHeaderName::as_str)
}

fn header_values<'a>(key: &'a str, headers: &'a IsahcHeaderMap) -> impl Iterator<Item=&'a str> + 'a {
    headers.get(key)
        .and_then(|k| k.to_str().ok()).unwrap()
        .split(',').map(|s| s.trim())
}
