use isahc::{AsyncBody as IsahcAsyncBody, Body as IsahcBody, Error as IsahcError, http::header::HeaderMap as IsahcHeaderMap, http::header::HeaderName as IsahcHeaderName, Response as IsahcResponse};

use super::super::{
    AsserhttpHeader,
    asserter::header::{assert_header_key, assert_header_key_absent, assert_header_value, assert_header_values},
};

impl AsserhttpHeader<IsahcResponse<IsahcBody>> for IsahcResponse<IsahcBody> {
    fn expect_header(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.as_ref());
        assert_header_value(header_values(key.as_ref(), self.headers()), key.as_ref(), value.as_ref());
        self
    }

    fn expect_headers<'a>(&mut self, key: impl AsRef<str>, value: impl Into<Vec<&'a str>>) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.as_ref());
        assert_header_values(header_values(key.as_ref(), self.headers()), key.as_ref(), value.into());
        self
    }

    fn expect_header_present(&mut self, key: impl AsRef<str>) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.as_ref());
        self
    }

    fn expect_header_absent(&mut self, key: impl AsRef<str>) -> &mut Self {
        assert_header_key_absent(header_keys(self.headers()), key.as_ref());
        self
    }
}

impl AsserhttpHeader<IsahcResponse<IsahcAsyncBody>> for IsahcResponse<IsahcAsyncBody> {
    fn expect_header(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.as_ref());
        assert_header_value(header_values(key.as_ref(), self.headers()), key.as_ref(), value.as_ref());
        self
    }

    fn expect_headers<'a>(&mut self, key: impl AsRef<str>, value: impl Into<Vec<&'a str>>) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.as_ref());
        assert_header_values(header_values(key.as_ref(), self.headers()), key.as_ref(), value.into());
        self
    }

    fn expect_header_present(&mut self, key: impl AsRef<str>) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.as_ref());
        self
    }

    fn expect_header_absent(&mut self, key: impl AsRef<str>) -> &mut Self {
        assert_header_key_absent(header_keys(self.headers()), key.as_ref());
        self
    }
}

impl AsserhttpHeader<IsahcResponse<IsahcBody>> for Result<IsahcResponse<IsahcBody>, IsahcError> {
    fn expect_header(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut IsahcResponse<IsahcBody> {
        self.as_mut().unwrap().expect_header(key, value)
    }

    fn expect_headers<'a>(&mut self, key: impl AsRef<str>, value: impl Into<Vec<&'a str>>) -> &mut IsahcResponse<IsahcBody> {
        self.as_mut().unwrap().expect_headers(key, value)
    }

    fn expect_header_present(&mut self, key: impl AsRef<str>) -> &mut IsahcResponse<IsahcBody> {
        self.as_mut().unwrap().expect_header_present(key)
    }

    fn expect_header_absent(&mut self, key: impl AsRef<str>) -> &mut IsahcResponse<IsahcBody> {
        self.as_mut().unwrap().expect_header_absent(key)
    }
}

impl AsserhttpHeader<IsahcResponse<IsahcAsyncBody>> for Result<IsahcResponse<IsahcAsyncBody>, IsahcError> {
    fn expect_header(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut IsahcResponse<IsahcAsyncBody> {
        self.as_mut().unwrap().expect_header(key, value)
    }

    fn expect_headers<'a>(&mut self, key: impl AsRef<str>, value: impl Into<Vec<&'a str>>) -> &mut IsahcResponse<IsahcAsyncBody> {
        self.as_mut().unwrap().expect_headers(key, value)
    }

    fn expect_header_present(&mut self, key: impl AsRef<str>) -> &mut IsahcResponse<IsahcAsyncBody> {
        self.as_mut().unwrap().expect_header_present(key)
    }

    fn expect_header_absent(&mut self, key: impl AsRef<str>) -> &mut IsahcResponse<IsahcAsyncBody> {
        self.as_mut().unwrap().expect_header_absent(key)
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
