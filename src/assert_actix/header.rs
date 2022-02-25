use actix_http::{body::Body as ActixBody, Error as ActixError, Response as ActixResponse};
use actix_web::dev::ServiceResponse as ActixServiceResponse;

use super::super::{
    AsserhttpHeader,
    asserter::header::{assert_header_key, assert_header_key_absent, assert_header_value, assert_header_values},
};

impl AsserhttpHeader<ActixResponse<ActixBody>> for ActixResponse<ActixBody> {
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

impl AsserhttpHeader<ActixResponse<ActixBody>> for Result<ActixResponse<ActixBody>, ActixError> {
    fn expect_header<'a>(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut ActixResponse<ActixBody> {
        self.as_mut().unwrap().expect_header(key, value)
    }

    fn expect_headers<'a, V: Into<Vec<&'a str>>>(&mut self, key: impl AsRef<str>, value: V) -> &mut ActixResponse<ActixBody> {
        self.as_mut().unwrap().expect_headers(key, value)
    }

    fn expect_header_present<'a>(&mut self, key: impl AsRef<str>) -> &mut ActixResponse<ActixBody> {
        self.as_mut().unwrap().expect_header_present(key)
    }

    fn expect_header_absent<'a>(&mut self, key: impl AsRef<str>) -> &mut ActixResponse<ActixBody> {
        self.as_mut().unwrap().expect_header_absent(key)
    }
}

impl AsserhttpHeader<ActixServiceResponse<ActixBody>> for ActixServiceResponse<ActixBody> {
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