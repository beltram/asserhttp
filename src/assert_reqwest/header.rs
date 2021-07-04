use reqwest::{
    blocking::Response as ReqwestResponse,
    Error as ReqwestError,
    Response as AsyncReqwestResponse,
    header::HeaderMap as ReqwestHeaderMap,
};

use super::super::{
    AsserhttpHeader,
    asserter::header::{assert_header_key, assert_header_key_absent, assert_header_value, assert_header_values},
};

impl AsserhttpHeader<ReqwestResponse> for ReqwestResponse {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut Self {
        let key = key.into();
        assert_header_key(header_keys(self.headers()), key);
        assert_header_value(header_values(key, self.headers()), key, value.into());
        self
    }

    fn expect_headers<'a, K: Into<&'a str>, V: Into<Vec<&'a str>>>(&mut self, key: K, value: V) -> &mut Self {
        let key = key.into();
        assert_header_key(header_keys(self.headers()), key);
        assert_header_values(header_values(key, self.headers()), key, value.into());
        self
    }

    fn expect_header_present<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.into());
        self
    }

    fn expect_header_absent<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut Self {
        assert_header_key_absent(header_keys(self.headers()), key.into());
        self
    }
}

impl AsserhttpHeader<AsyncReqwestResponse> for AsyncReqwestResponse {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut Self {
        let key = key.into();
        assert_header_key(header_keys(self.headers()), key);
        assert_header_value(header_values(key, self.headers()), key, value.into());
        self
    }

    fn expect_headers<'a, K: Into<&'a str>, V: Into<Vec<&'a str>>>(&mut self, key: K, value: V) -> &mut Self {
        let key = key.into();
        assert_header_key(header_keys(self.headers()), key);
        assert_header_values(header_values(key, self.headers()), key, value.into());
        self
    }

    fn expect_header_present<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.into());
        self
    }

    fn expect_header_absent<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut Self {
        assert_header_key_absent(header_keys(self.headers()), key.into());
        self
    }
}

impl AsserhttpHeader<ReqwestResponse> for Result<ReqwestResponse, ReqwestError> {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut ReqwestResponse {
        self.as_mut().unwrap().expect_header(key, value)
    }

    fn expect_headers<'a, K: Into<&'a str>, V: Into<Vec<&'a str>>>(&mut self, key: K, value: V) -> &mut ReqwestResponse {
        self.as_mut().unwrap().expect_headers(key, value)
    }

    fn expect_header_present<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut ReqwestResponse {
        self.as_mut().unwrap().expect_header_present(key)
    }

    fn expect_header_absent<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut ReqwestResponse {
        self.as_mut().unwrap().expect_header_absent(key)
    }
}

impl AsserhttpHeader<AsyncReqwestResponse> for Result<AsyncReqwestResponse, ReqwestError> {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut AsyncReqwestResponse {
        self.as_mut().unwrap().expect_header(key, value)
    }

    fn expect_headers<'a, K: Into<&'a str>, V: Into<Vec<&'a str>>>(&mut self, key: K, value: V) -> &mut AsyncReqwestResponse {
        self.as_mut().unwrap().expect_headers(key, value)
    }

    fn expect_header_present<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut AsyncReqwestResponse {
        self.as_mut().unwrap().expect_header_present(key)
    }

    fn expect_header_absent<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut AsyncReqwestResponse {
        self.as_mut().unwrap().expect_header_absent(key)
    }
}

fn header_keys(headers: &ReqwestHeaderMap) -> impl Iterator<Item=&str> {
    headers.keys().map(|it| it.as_str())
}

fn header_values<'a>(key: &'a str, headers: &'a ReqwestHeaderMap) -> impl Iterator<Item=&'a str> + 'a {
    headers.get(key)
        .and_then(|k| k.to_str().ok()).unwrap()
        .split(',').map(|s| s.trim())
}
