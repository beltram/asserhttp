use rocket::http::HeaderMap as RocketHeaderMap;
use rocket::local::{
    asynchronous::LocalResponse as AsyncRocketResponse,
    blocking::LocalResponse as BlockingRocketResponse,
};

use super::super::{
    AsserhttpHeader,
    asserter::header::{assert_header_key, assert_header_key_absent, assert_header_value, assert_header_values},
};

impl <'b> AsserhttpHeader<BlockingRocketResponse<'b>> for BlockingRocketResponse<'b> {
    fn expect_header(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.as_ref().to_string());
        assert_header_value(header_values(key.as_ref(), self.headers()), key.as_ref(), value.as_ref());
        self
    }

    fn expect_headers<'a>(&mut self, key: impl AsRef<str>, value: impl Into<Vec<&'a str>>) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.as_ref().to_string());
        assert_header_values(header_values(key.as_ref(), self.headers()), key.as_ref(), value.into());
        self
    }

    fn expect_header_present(&mut self, key: impl AsRef<str>) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.as_ref().to_string());
        self
    }

    fn expect_header_absent(&mut self, key: impl AsRef<str>) -> &mut Self {
        assert_header_key_absent(header_keys(self.headers()), key.as_ref().to_string());
        self
    }
}

impl <'b> AsserhttpHeader<AsyncRocketResponse<'b>> for AsyncRocketResponse<'b> {
    fn expect_header(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.as_ref().to_string());
        assert_header_value(header_values(key.as_ref(), self.headers()), key.as_ref(), value.as_ref());
        self
    }

    fn expect_headers<'a>(&mut self, key: impl AsRef<str>, value: impl Into<Vec<&'a str>>) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.as_ref().to_string());
        assert_header_values(header_values(key.as_ref(), self.headers()), key.as_ref(), value.into());
        self
    }

    fn expect_header_present(&mut self, key: impl AsRef<str>) -> &mut Self {
        assert_header_key(header_keys(self.headers()), key.as_ref().to_string());
        self
    }

    fn expect_header_absent(&mut self, key: impl AsRef<str>) -> &mut Self {
        assert_header_key_absent(header_keys(self.headers()), key.as_ref().to_string());
        self
    }
}

fn header_keys<'a>(headers: &'a RocketHeaderMap) -> impl Iterator<Item=String> + 'a {
    headers.iter().map(|it| it.name.to_string())
}

fn header_values<'a>(key: &'a str, headers: &'a RocketHeaderMap) -> impl Iterator<Item=&'a str> + 'a {
    headers.get(key)
}
