use crate::{
    accessor::HeaderAccessor,
    error::{AsserhttpError, AsserhttpResult},
    header::{
        infallible::{HeaderValueAsserter, HeaderValuesAsserter},
        key::HeaderKey,
        value::HeaderValue,
        values::HeaderValues,
    },
};

/// For assertions on http response headers returning an error instead of panicking
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
    ///     reqwest::blocking::get("http://localhost").try_expect_header("content-type", "application/json").unwrap();
    ///     reqwest::blocking::get("http://localhost").try_expect_header(headers::CONTENT_TYPE, "application/json").unwrap();
    ///     reqwest::blocking::get("http://localhost").try_expect_header("content-type", |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///     reqwest::blocking::get("http://localhost").try_expect_header(headers::CONTENT_TYPE, |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_header("content-type", "application/json").unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_header(headers::CONTENT_TYPE, "application/json").unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_header("content-type", |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_header(headers::CONTENT_TYPE, |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_header("content-type", "application/json").unwrap();
    ///     isahc::get("http://localhost").try_expect_header(headers::CONTENT_TYPE, "application/json").unwrap();
    ///     isahc::get("http://localhost").try_expect_header("content-type", |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///     isahc::get("http://localhost").try_expect_header(headers::CONTENT_TYPE, |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_header("content-type", "application/json").unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_header(headers::CONTENT_TYPE, "application/json").unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_header("content-type", |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_header(headers::CONTENT_TYPE, |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_header("content-type", "application/json").unwrap();
    ///     surf::get("http://localhost").await.try_expect_header(headers::CONTENT_TYPE, "application/json").unwrap();
    ///     surf::get("http://localhost").await.try_expect_header("content-type", |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///     surf::get("http://localhost").await.try_expect_header(headers::CONTENT_TYPE, |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_header("content-type", "application/json").unwrap();
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_header(headers::CONTENT_TYPE, "application/json").unwrap();
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_header("content-type", |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_header(headers::CONTENT_TYPE, |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_header("content-type", "application/json").unwrap();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_header(headers::CONTENT_TYPE, "application/json").unwrap();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_header("content-type", |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_header(headers::CONTENT_TYPE, |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_header("content-type", "application/json").unwrap();
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_header(headers::CONTENT_TYPE, "application/json").unwrap();
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_header("content-type", |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_header(headers::CONTENT_TYPE, |h: &str| { assert!(h.contains("application/")); Ok(()) }).unwrap();
    /// }
    /// ```
    ///
    /// # Error
    /// * [AsserhttpError::HeaderAbsent] when expected header is not found by key
    /// * [AsserhttpError::HeaderValueMismatch] when expected header is found by key but the value mismatches
    /// * [AsserhttpError::MultivaluedHeader] when a multivalued header is found
    fn try_expect_header(&mut self, key: impl Into<HeaderKey>, value: impl Into<FallibleHeaderValueAsserter>) -> AsserhttpResult<&mut T>;

    /// Expects response multivalued headers to be equal
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
    ///     reqwest::blocking::get("http://localhost").try_expect_headers("cache-control", ["no-cache", "no-store"]).unwrap();
    ///     reqwest::blocking::get("http://localhost").try_expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]).unwrap();
    ///     reqwest::blocking::get("http://localhost").try_expect_headers("cache-control", |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///     reqwest::blocking::get("http://localhost").try_expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_headers("cache-control", ["no-cache", "no-store"]).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_headers("cache-control", |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_headers("cache-control", ["no-cache", "no-store"]).unwrap();
    ///     isahc::get("http://localhost").try_expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]).unwrap();
    ///     isahc::get("http://localhost").try_expect_headers("cache-control", |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///     isahc::get("http://localhost").try_expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_headers("cache-control", ["no-cache", "no-store"]).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_headers("cache-control", |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_headers("cache-control", ["no-cache", "no-store"]).unwrap();
    ///     surf::get("http://localhost").await.try_expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]).unwrap();
    ///     surf::get("http://localhost").await.try_expect_headers("cache-control", |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///     surf::get("http://localhost").await.try_expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_headers("cache-control", ["no-cache", "no-store"]).unwrap();
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]).unwrap();
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_headers("cache-control", |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_headers("cache-control", ["no-cache", "no-store"]).unwrap();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]).unwrap();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_headers("cache-control", |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_headers("cache-control", ["no-cache", "no-store"]).unwrap();
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"]).unwrap();
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_headers("cache-control", |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_headers(headers::CACHE_CONTROL, |h: Vec<&str>| { assert!(h.contains(&"no-cache") && h.contains(&"no-store")); Ok(()) }).unwrap();
    /// }
    /// ```
    /// # Error
    /// * [AsserhttpError::HeaderAbsent] when expected header is not found by key
    /// * [AsserhttpError::HeaderValuesMismatch] when expected header is found by key but the values mismatch
    /// * [AsserhttpError::InvalidHeaderValuesSupplied] when you supplied an empty list of expected header values
    fn try_expect_headers(&mut self, key: impl Into<HeaderKey>, values: impl Into<FallibleHeaderValuesAsserter>)
        -> AsserhttpResult<&mut T>;

    /// Expects response header to be present
    /// * `key` - expected present header key
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").try_expect_header_present("content-type").unwrap();
    ///     reqwest::blocking::get("http://localhost").try_expect_header_present(headers::CONTENT_TYPE).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_header_present("content-type").unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_header_present(headers::CONTENT_TYPE).unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_header_present("content-type").unwrap();
    ///     isahc::get("http://localhost").try_expect_header_present(headers::CONTENT_TYPE).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_header_present("content-type").unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_header_present(headers::CONTENT_TYPE).unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_header_present("content-type").unwrap();
    ///     surf::get("http://localhost").await.try_expect_header_present(headers::CONTENT_TYPE).unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_header_present("content-type").unwrap();
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_header_present(headers::CONTENT_TYPE).unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_header_present("content-type").unwrap();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_header_present(headers::CONTENT_TYPE).unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_header_present("content-type").unwrap();
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_header_present(headers::CONTENT_TYPE).unwrap();
    /// }
    /// ```
    /// # Error
    /// * [AsserhttpError::HeaderAbsent] when expected header is not found
    fn try_expect_header_present(&mut self, key: impl Into<HeaderKey>) -> AsserhttpResult<&mut T>;

    /// Expects response header to be absent
    /// * `key` - expected absent header key
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").try_expect_header_absent("content-type").unwrap();
    ///     reqwest::blocking::get("http://localhost").try_expect_header_absent(headers::CONTENT_TYPE).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_header_absent("content-type").unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_header_absent(headers::CONTENT_TYPE).unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_header_absent("content-type").unwrap();
    ///     isahc::get("http://localhost").try_expect_header_absent(headers::CONTENT_TYPE).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_header_absent("content-type").unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_header_absent(headers::CONTENT_TYPE).unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_header_absent("content-type").unwrap();
    ///     surf::get("http://localhost").await.try_expect_header_absent(headers::CONTENT_TYPE).unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_header_absent("content-type").unwrap();
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_header_absent(headers::CONTENT_TYPE).unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_header_absent("content-type").unwrap();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_header_absent(headers::CONTENT_TYPE).unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_header_absent("content-type").unwrap();
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_header_absent(headers::CONTENT_TYPE).unwrap();
    /// }
    /// ```
    /// # Error
    /// * [AsserhttpError::HeaderPresent] when expected header is found by key
    fn try_expect_header_absent(&mut self, key: impl Into<HeaderKey>) -> AsserhttpResult<&mut T>;
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

    fn try_expect_headers(
        &mut self, key: impl Into<HeaderKey>, values: impl Into<FallibleHeaderValuesAsserter>,
    ) -> AsserhttpResult<&mut T> {
        let key = key.into();
        key.try_assert_contained(self.get_keys())?;
        let actual_values = self.get_values(&key);
        values.into()(key, actual_values)?;
        Ok(self)
    }

    fn try_expect_header_present(&mut self, key: impl Into<HeaderKey>) -> AsserhttpResult<&mut T> {
        let key = key.into();
        key.try_assert_contained(self.get_keys())?;
        Ok(self)
    }

    fn try_expect_header_absent(&mut self, key: impl Into<HeaderKey>) -> AsserhttpResult<&mut T> {
        let key = key.into();
        key.try_assert_absent(self.get_keys())?;
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

    fn try_expect_headers(
        &mut self, key: impl Into<HeaderKey>, values: impl Into<FallibleHeaderValuesAsserter>,
    ) -> AsserhttpResult<&mut T> {
        self.as_mut()
            .map_err(|e| AsserhttpError::HttpError(format!("{e:?}")))?
            .try_expect_headers(key, values)
    }

    fn try_expect_header_present(&mut self, key: impl Into<HeaderKey>) -> AsserhttpResult<&mut T> {
        self.as_mut()
            .map_err(|e| AsserhttpError::HttpError(format!("{e:?}")))?
            .try_expect_header_present(key)
    }

    fn try_expect_header_absent(&mut self, key: impl Into<HeaderKey>) -> AsserhttpResult<&mut T> {
        self.as_mut()
            .map_err(|e| AsserhttpError::HttpError(format!("{e:?}")))?
            .try_expect_header_absent(key)
    }
}

pub struct FallibleHeaderValueAsserter(Box<dyn Fn(HeaderKey, HeaderValue) -> AsserhttpResult<()>>);

impl<'a> From<&'a String> for FallibleHeaderValueAsserter {
    fn from(expected: &'a String) -> Self {
        HeaderValue::from(expected).into()
    }
}

impl<'a> From<&'a str> for FallibleHeaderValueAsserter {
    fn from(expected: &'a str) -> Self {
        HeaderValue::from(expected).into()
    }
}

impl From<String> for FallibleHeaderValueAsserter {
    fn from(expected: String) -> Self {
        HeaderValue::from(expected).into()
    }
}

impl From<HeaderValueAsserter> for FallibleHeaderValueAsserter {
    fn from(asserter: HeaderValueAsserter) -> Self {
        Self(Box::new(move |key, value| {
            asserter(key, value);
            Ok(())
        }))
    }
}

impl<F: 'static> From<F> for FallibleHeaderValueAsserter
where
    F: Fn(&'static str) -> AsserhttpResult<()>,
{
    fn from(fun: F) -> Self {
        Self(Box::new(move |_, value| fun(Box::leak(Box::new(value)))))
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

impl std::ops::Deref for FallibleHeaderValueAsserter {
    type Target = dyn Fn(HeaderKey, HeaderValue) -> AsserhttpResult<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct FallibleHeaderValuesAsserter(Box<dyn Fn(HeaderKey, HeaderValues) -> AsserhttpResult<()>>);

impl<S: AsRef<str>> From<Vec<S>> for FallibleHeaderValuesAsserter {
    fn from(expected: Vec<S>) -> Self {
        HeaderValues::from(expected).into()
    }
}

impl<const N: usize, S: AsRef<str>> From<[S; N]> for FallibleHeaderValuesAsserter {
    fn from(expected: [S; N]) -> Self {
        HeaderValues::from(expected).into()
    }
}

impl<'a, const N: usize, S: AsRef<str>> From<&'a [S; N]> for FallibleHeaderValuesAsserter {
    fn from(expected: &'a [S; N]) -> Self {
        HeaderValues::from(Vec::from_iter(expected.iter())).into()
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

impl From<HeaderValues> for FallibleHeaderValuesAsserter {
    fn from(expected: HeaderValues) -> Self {
        Self(Box::new(move |key, actual_values| {
            if expected.is_empty() {
                return Err(AsserhttpError::InvalidHeaderValuesSupplied { key });
            }
            if expected != actual_values {
                return Err(AsserhttpError::HeaderValuesMismatch {
                    key,
                    actual_values,
                    expected: expected.clone(),
                });
            }
            Ok(())
        }))
    }
}

impl std::ops::Deref for FallibleHeaderValuesAsserter {
    type Target = dyn Fn(HeaderKey, HeaderValues) -> AsserhttpResult<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HeaderValuesAsserter> for FallibleHeaderValuesAsserter {
    fn from(asserter: HeaderValuesAsserter) -> Self {
        Self(Box::new(move |key, values| {
            asserter(key, values);
            Ok(())
        }))
    }
}
