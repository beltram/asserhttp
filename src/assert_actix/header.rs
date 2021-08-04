use actix_http::{body::Body as ActixBody, Response as ActixResponse, Error as ActixError};
use actix_web::dev::ServiceResponse as ActixServiceResponse;

use super::super::{
    AsserhttpHeader,
    asserter::header::{assert_header_key, assert_header_key_absent, assert_header_value, assert_header_values},
};

impl AsserhttpHeader<ActixResponse<ActixBody>> for ActixResponse<ActixBody> {
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

impl AsserhttpHeader<ActixResponse<ActixBody>> for Result<ActixResponse<ActixBody>, ActixError> {
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut ActixResponse<ActixBody> {
        self.as_mut().unwrap().expect_header(key, value)
    }

    fn expect_headers<'a, K: Into<&'a str>, V: Into<Vec<&'a str>>>(&mut self, key: K, value: V) -> &mut ActixResponse<ActixBody> {
        self.as_mut().unwrap().expect_headers(key, value)
    }

    fn expect_header_present<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut ActixResponse<ActixBody> {
        self.as_mut().unwrap().expect_header_present(key)
    }

    fn expect_header_absent<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut ActixResponse<ActixBody> {
        self.as_mut().unwrap().expect_header_absent(key)
    }
}

impl AsserhttpHeader<ActixServiceResponse<ActixBody>> for ActixServiceResponse<ActixBody> {
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