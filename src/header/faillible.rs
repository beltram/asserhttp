use crate::{
    header::{value::HeaderValue, key::HeaderKey},
    accessor::HeaderAccessor,
    error::{AsserhttpError, AsserhttpResult},
};
use crate::header::values::HeaderValues;

/// For assertions on http response headers
pub trait FallibleAsserhttpHeader<T> {
    /// Expects response header to be equal
    /// * `key` - expected header key
    /// * `value` - expected header value or closure
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_header("content-type", "application/json");
    ///     reqwest::blocking::get("http://localhost").expect_header(headers::CONTENT_TYPE, "application/json");
    ///     reqwest::blocking::get("http://localhost").expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///     reqwest::blocking::get("http://localhost").expect_header(headers::CONTENT_TYPE, |h: &str| assert!(h.contains("application/")));
    ///     reqwest::get("http://localhost").await.expect_header("content-type", "application/json");
    ///     reqwest::get("http://localhost").await.expect_header(headers::CONTENT_TYPE, "application/json");
    ///     reqwest::get("http://localhost").await.expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///     reqwest::get("http://localhost").await.expect_header(headers::CONTENT_TYPE, |h: &str| assert!(h.contains("application/")));
    ///
    ///     isahc::get("http://localhost").expect_header("content-type", "application/json");
    ///     isahc::get("http://localhost").expect_header(headers::CONTENT_TYPE, "application/json");
    ///     isahc::get("http://localhost").expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///     isahc::get("http://localhost").expect_header(headers::CONTENT_TYPE, |h: &str| assert!(h.contains("application/")));
    ///     isahc::get_async("http://localhost").await.expect_header("content-type", "application/json");
    ///     isahc::get_async("http://localhost").await.expect_header(headers::CONTENT_TYPE, "application/json");
    ///     isahc::get_async("http://localhost").await.expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///     isahc::get_async("http://localhost").await.expect_header(headers::CONTENT_TYPE, |h: &str| assert!(h.contains("application/")));
    ///
    ///     surf::get("http://localhost").await.expect_header("content-type", "application/json");
    ///     surf::get("http://localhost").await.expect_header(headers::CONTENT_TYPE, "application/json");
    ///     surf::get("http://localhost").await.expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///     surf::get("http://localhost").await.expect_header(headers::CONTENT_TYPE, |h: &str| assert!(h.contains("application/")));
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_header("content-type", "application/json");
    ///     ureq::get("http://localhost").call().or_any_status().expect_header(headers::CONTENT_TYPE, "application/json");
    ///     ureq::get("http://localhost").call().or_any_status().expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///     ureq::get("http://localhost").call().or_any_status().expect_header(headers::CONTENT_TYPE, |h: &str| assert!(h.contains("application/")));
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header("content-type", "application/json");
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header(headers::CONTENT_TYPE, "application/json");
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header(headers::CONTENT_TYPE, |h: &str| assert!(h.contains("application/")));
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_header("content-type", "application/json");
    ///     awc::Client::default().get("http://localhost").send().await.expect_header(headers::CONTENT_TYPE, "application/json");
    ///     awc::Client::default().get("http://localhost").send().await.expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///     awc::Client::default().get("http://localhost").send().await.expect_header(headers::CONTENT_TYPE, |h: &str| assert!(h.contains("application/")));
    /// }
    /// ```
    fn try_expect_header(&mut self, key: impl Into<HeaderKey>, value: impl Into<FallibleHeaderValueAsserter>) -> AsserhttpResult<&mut T>;

    /// Expects response multi valued headers to be equal
    /// * `key` - expected header key
    /// * `value` - expected header values or closure
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     reqwest::blocking::get("http://localhost").expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]);
    ///     reqwest::blocking::get("http://localhost").expect_headers("cache-control", |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///     reqwest::blocking::get("http://localhost").expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///     reqwest::get("http://localhost").await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     reqwest::get("http://localhost").await.expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]);
    ///     reqwest::get("http://localhost").await.expect_headers("cache-control", |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///     reqwest::get("http://localhost").await.expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///
    ///     isahc::get("http://localhost").expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     isahc::get("http://localhost").expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]);
    ///     isahc::get("http://localhost").expect_headers("cache-control", |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///     isahc::get("http://localhost").expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///     isahc::get_async("http://localhost").await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     isahc::get_async("http://localhost").await.expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]);
    ///     isahc::get_async("http://localhost").await.expect_headers("cache-control", |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///     isahc::get_async("http://localhost").await.expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///
    ///     surf::get("http://localhost").await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     surf::get("http://localhost").await.expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]);
    ///     surf::get("http://localhost").await.expect_headers("cache-control", |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///     surf::get("http://localhost").await.expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     ureq::get("http://localhost").call().or_any_status().expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]);
    ///     ureq::get("http://localhost").call().or_any_status().expect_headers("cache-control", |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///     ureq::get("http://localhost").call().or_any_status().expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]);
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_headers("cache-control", |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     awc::Client::default().get("http://localhost").send().await.expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]);
    ///     awc::Client::default().get("http://localhost").send().await.expect_headers("cache-control", |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    ///     awc::Client::default().get("http://localhost").send().await.expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| assert!(h.contains(&"no-cache") && h.contains(&"no-store")));
    /// }
    /// ```
    fn try_expect_headers(&mut self, key: impl Into<HeaderKey>, values: impl Into<FallibleHeaderValuesAsserter>) -> AsserhttpResult<&mut T>;
}

impl<T> FallibleAsserhttpHeader<T> for T
    where
        T: HeaderAccessor,
{
    fn try_expect_header(&mut self, key: impl Into<HeaderKey>, value: impl Into<FallibleHeaderValueAsserter>) -> AsserhttpResult<&mut T> {
        let key = key.into();
        key.try_assert_contained(self.get_keys())?;
        let actual_values = self.get_values(&key);
        let values_count = actual_values.len();
        if values_count > 1 {
            return Err(AsserhttpError::MultivaluedHeader {
                key,
                values_count,
                actual_values,
            });
        }
        let actual_value = actual_values.first().ok_or(AsserhttpError::InternalError)?.into();
        value.into()(key, actual_value)?;
        Ok(self)
    }

    fn try_expect_headers(&mut self, key: impl Into<HeaderKey>, values: impl Into<FallibleHeaderValuesAsserter>) -> AsserhttpResult<&mut T> {
        let key = key.into();
        key.try_assert_contained(self.get_keys())?;
        let actual_values = self.get_values(&key);
        values.into()(key, actual_values)?;
        Ok(self)
    }
}

impl<T, E> FallibleAsserhttpHeader<T> for Result<T, E>
    where
        T: HeaderAccessor,
        E: std::fmt::Debug,
{
    fn try_expect_header(&mut self, key: impl Into<HeaderKey>, value: impl Into<FallibleHeaderValueAsserter>) -> AsserhttpResult<&mut T> {
        self.as_mut()
            .map_err(|e| AsserhttpError::HttpError(format!("{e:?}")))?
            .try_expect_header(key, value)
    }

    fn try_expect_headers(&mut self, key: impl Into<HeaderKey>, values: impl Into<FallibleHeaderValuesAsserter>) -> AsserhttpResult<&mut T> {
        self.as_mut()
            .map_err(|e| AsserhttpError::HttpError(format!("{e:?}")))?
            .try_expect_headers(key, values)
    }
}

pub struct FallibleHeaderValueAsserter(Box<dyn Fn(HeaderKey, HeaderValue) -> AsserhttpResult<()>>);

impl<'a> From<&'a String> for FallibleHeaderValueAsserter {
    fn from(expected: &'a String) -> Self {
        Self::from(expected.as_str())
    }
}

impl<'a> From<&'a str> for FallibleHeaderValueAsserter {
    fn from(expected: &'a str) -> Self {
        Self::from(expected.to_string())
    }
}

impl From<String> for FallibleHeaderValueAsserter {
    fn from(expected: String) -> Self {
        expected.into()
    }
}

impl From<HeaderValue> for FallibleHeaderValueAsserter {
    fn from(expected: HeaderValue) -> Self {
        Self(Box::new(move |key, actual| {
            if actual != expected {
                return Err(AsserhttpError::HeaderValueMismatch {
                    key,
                    actual,
                    expected: expected.clone(),
                });
            }
            Ok(())
        }))
    }
}

impl<F: 'static> From<F> for FallibleHeaderValueAsserter
    where
        F: Fn(&'static str),
{
    fn from(fun: F) -> Self {
        Self(Box::new(move |_, value| {
            fun(Box::leak(Box::new(value)));
            Ok(())
        }))
    }
}

impl std::ops::Deref for FallibleHeaderValueAsserter {
    type Target = dyn Fn(HeaderKey, HeaderValue) -> AsserhttpResult<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct FallibleHeaderValuesAsserter(Box<dyn Fn(HeaderKey, HeaderValues) -> AsserhttpResult<()>>);

impl<S: AsRef<str>> From<Vec<S>> for FallibleHeaderValuesAsserter {
    fn from(expected: Vec<S>) -> Self {
        expected.into()
    }
}

impl From<HeaderValues> for FallibleHeaderValuesAsserter {
    fn from(expected: HeaderValues) -> Self {
        Self(Box::new(move |key, actual_values| {
            if expected.is_empty() {
                return Err(AsserhttpError::InvalidHeaderValuesSupplied { key });
            }
            if expected != actual_values {
                return Err(AsserhttpError::HeaderValuesMismatch { key, actual_values, expected: expected.clone() });
            }
            Ok(())
        }))
    }
}

impl<const N: usize, S: AsRef<str>> From<[S; N]> for FallibleHeaderValuesAsserter {
    fn from(expected: [S; N]) -> Self {
        expected.into()
    }
}

impl<'a, const N: usize, S: AsRef<str>> From<&'a [S; N]> for FallibleHeaderValuesAsserter {
    fn from(expected: &'a [S; N]) -> Self {
        Vec::from_iter(expected.iter()).into()
    }
}

impl<F: 'static> From<F> for FallibleHeaderValuesAsserter
    where
        F: Fn(Vec<&'static str>) -> AsserhttpResult<()>,
{
    fn from(fun: F) -> Self {
        Self(Box::new(move |_, actual_values| {
            fun(actual_values
                .iter()
                .map(|s| Box::leak(Box::new(s.clone())).as_str())
                .collect::<Vec<_>>())
        }))
    }
}

impl std::ops::Deref for FallibleHeaderValuesAsserter {
    type Target = dyn Fn(HeaderKey, HeaderValues) -> AsserhttpResult<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
