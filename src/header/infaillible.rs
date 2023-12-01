use crate::{
    accessor::HeaderAccessor,
    header::{key::HeaderKey, value::HeaderValue, values::HeaderValues},
};

/// For assertions on http response headers
pub trait AsserhttpHeader<T> {
    const APPLICATION_JSON: &'static str = "application/json";
    const TEXT_PLAIN: &'static str = "text/plain";

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
    fn expect_header(&mut self, key: impl Into<HeaderKey>, value: impl Into<HeaderValueAsserter>) -> &mut T;

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
    fn expect_headers(&mut self, key: impl Into<HeaderKey>, values: impl Into<HeaderValuesAsserter>) -> &mut T;

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
    ///     reqwest::blocking::get("http://localhost").expect_header_present("content-type");
    ///     reqwest::blocking::get("http://localhost").expect_header_present(headers::CONTENT_TYPE);
    ///     reqwest::get("http://localhost").await.expect_header_present("content-type");
    ///     reqwest::get("http://localhost").await.expect_header_present(headers::CONTENT_TYPE);
    ///
    ///     isahc::get("http://localhost").expect_header_present("content-type");
    ///     isahc::get("http://localhost").expect_header_present(headers::CONTENT_TYPE);
    ///     isahc::get_async("http://localhost").await.expect_header_present("content-type");
    ///     isahc::get_async("http://localhost").await.expect_header_present(headers::CONTENT_TYPE);
    ///
    ///     surf::get("http://localhost").await.expect_header_present("content-type");
    ///     surf::get("http://localhost").await.expect_header_present(headers::CONTENT_TYPE);
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_header_present("content-type");
    ///     ureq::get("http://localhost").call().or_any_status().expect_header_present(headers::CONTENT_TYPE);
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header_present("content-type");
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header_present(headers::CONTENT_TYPE);
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_header_present("content-type");
    ///     awc::Client::default().get("http://localhost").send().await.expect_header_present(headers::CONTENT_TYPE);
    /// }
    /// ```
    fn expect_header_present(&mut self, key: impl Into<HeaderKey>) -> &mut T;

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
    ///     reqwest::blocking::get("http://localhost").expect_header_absent("content-type");
    ///     reqwest::blocking::get("http://localhost").expect_header_absent(headers::CONTENT_TYPE);
    ///     reqwest::get("http://localhost").await.expect_header_absent("content-type");
    ///     reqwest::get("http://localhost").await.expect_header_absent(headers::CONTENT_TYPE);
    ///
    ///     isahc::get("http://localhost").expect_header_absent("content-type");
    ///     isahc::get("http://localhost").expect_header_absent(headers::CONTENT_TYPE);
    ///     isahc::get_async("http://localhost").await.expect_header_absent("content-type");
    ///     isahc::get_async("http://localhost").await.expect_header_absent(headers::CONTENT_TYPE);
    ///
    ///     surf::get("http://localhost").await.expect_header_absent("content-type");
    ///     surf::get("http://localhost").await.expect_header_absent(headers::CONTENT_TYPE);
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_header_absent("content-type");
    ///     ureq::get("http://localhost").call().or_any_status().expect_header_absent(headers::CONTENT_TYPE);
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header_absent("content-type");
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header_absent(headers::CONTENT_TYPE);
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_header_absent("content-type");
    ///     awc::Client::default().get("http://localhost").send().await.expect_header_absent(headers::CONTENT_TYPE);
    /// }
    /// ```
    fn expect_header_absent(&mut self, key: impl Into<HeaderKey>) -> &mut T;

    /// Expects response header `Content-Type: application/json`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_content_type_json();
    ///     reqwest::get("http://localhost").await.expect_content_type_json();
    ///
    ///     isahc::get("http://localhost").expect_content_type_json();
    ///     isahc::get_async("http://localhost").await.expect_content_type_json();
    ///
    ///     surf::get("http://localhost").await.expect_content_type_json();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_content_type_json();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_content_type_json();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_content_type_json();
    /// }
    /// ```
    fn expect_content_type_json(&mut self) -> &mut T {
        self.expect_header(crate::headers::CONTENT_TYPE, Self::APPLICATION_JSON)
    }

    /// Expects response header `Content-Type: text/plain`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_content_type_text();
    ///     reqwest::get("http://localhost").await.expect_content_type_text();
    ///
    ///     isahc::get("http://localhost").expect_content_type_text();
    ///     isahc::get_async("http://localhost").await.expect_content_type_text();
    ///
    ///     surf::get("http://localhost").await.expect_content_type_text();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_content_type_text();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_content_type_text();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_content_type_text();
    /// }
    /// ```
    fn expect_content_type_text(&mut self) -> &mut T {
        self.expect_header(crate::headers::CONTENT_TYPE, Self::TEXT_PLAIN)
    }
}

impl<T> AsserhttpHeader<T> for T
where
    T: HeaderAccessor,
{
    fn expect_header(&mut self, key: impl Into<HeaderKey>, value: impl Into<HeaderValueAsserter>) -> &mut T {
        let key = key.into();
        key.assert_contained(self.get_keys());
        let actual_values = self.get_values(&key);
        let values_count = actual_values.len();
        assert_eq!(
            values_count, 1,
            "expected header '{key}' to be single valued. Had '{values_count}' values '{actual_values:?}'. Use 'expect_headers' instead.",
        );
        let actual_value = actual_values.first().unwrap().into();
        value.into()(key, actual_value);
        self
    }

    fn expect_headers(&mut self, key: impl Into<HeaderKey>, values: impl Into<HeaderValuesAsserter>) -> &mut T {
        let key = key.into();
        key.assert_contained(self.get_keys());
        let actual_values = self.get_values(&key);
        values.into()(key, actual_values);
        self
    }

    fn expect_header_present(&mut self, key: impl Into<HeaderKey>) -> &mut T {
        key.into().assert_contained(self.get_keys());
        self
    }

    fn expect_header_absent(&mut self, key: impl Into<HeaderKey>) -> &mut T {
        key.into().assert_absent(self.get_keys());
        self
    }
}

impl<T, E> AsserhttpHeader<T> for Result<T, E>
where
    T: HeaderAccessor,
    E: std::fmt::Debug,
{
    fn expect_header(&mut self, key: impl Into<HeaderKey>, value: impl Into<HeaderValueAsserter>) -> &mut T {
        self.as_mut().unwrap().expect_header(key, value)
    }

    fn expect_headers(&mut self, key: impl Into<HeaderKey>, values: impl Into<HeaderValuesAsserter>) -> &mut T {
        self.as_mut().unwrap().expect_headers(key, values)
    }

    fn expect_header_present(&mut self, key: impl Into<HeaderKey>) -> &mut T {
        self.as_mut().unwrap().expect_header_present(key)
    }

    fn expect_header_absent(&mut self, key: impl Into<HeaderKey>) -> &mut T {
        self.as_mut().unwrap().expect_header_absent(key)
    }
}

pub struct HeaderValueAsserter(Box<dyn Fn(HeaderKey, HeaderValue)>);

impl<'a> From<&'a String> for HeaderValueAsserter {
    fn from(expected: &'a String) -> Self {
        HeaderValue::from(expected).into()
    }
}

impl<'a> From<&'a str> for HeaderValueAsserter {
    fn from(expected: &'a str) -> Self {
        HeaderValue::from(expected).into()
    }
}

impl From<String> for HeaderValueAsserter {
    fn from(expected: String) -> Self {
        HeaderValue::from(expected).into()
    }
}

impl<F: 'static> From<F> for HeaderValueAsserter
where
    F: Fn(&'static str),
{
    fn from(fun: F) -> Self {
        Self(Box::new(move |_, value| fun(Box::leak(Box::new(value)))))
    }
}

impl From<HeaderValue> for HeaderValueAsserter {
    fn from(expected: HeaderValue) -> Self {
        Self(Box::new(move |key, actual| {
            assert_eq!(
                actual, expected,
                "expected header '{key}' to be equal to '{expected}' but was '{actual}'"
            )
        }))
    }
}

impl std::ops::Deref for HeaderValueAsserter {
    type Target = dyn Fn(HeaderKey, HeaderValue);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct HeaderValuesAsserter(Box<dyn Fn(HeaderKey, HeaderValues)>);

impl<S: AsRef<str>> From<Vec<S>> for HeaderValuesAsserter {
    fn from(expected: Vec<S>) -> Self {
        HeaderValues::from(expected).into()
    }
}

impl<const N: usize, S: AsRef<str>> From<[S; N]> for HeaderValuesAsserter {
    fn from(expected: [S; N]) -> Self {
        HeaderValues::from(expected).into()
    }
}

impl<'a, const N: usize, S: AsRef<str>> From<&'a [S; N]> for HeaderValuesAsserter {
    fn from(expected: &'a [S; N]) -> Self {
        HeaderValues::from(Vec::from_iter(expected.iter())).into()
    }
}

impl<F: 'static> From<F> for HeaderValuesAsserter
where
    F: Fn(Vec<&'static str>),
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

impl From<HeaderValues> for HeaderValuesAsserter {
    fn from(expected: HeaderValues) -> Self {
        Self(Box::new(move |key, actual_values| {
            assert!(
                !expected.is_empty(),
                "no value expected for header '{key}'. Use 'expect_header_present' instead"
            );
            let same_size = actual_values.len() == expected.len();
            let all_eq = actual_values == expected;
            assert!(
                same_size && all_eq,
                "expected header '{key}' to contain values '{expected:?}' but contained '{actual_values}'"
            );
        }))
    }
}

impl std::ops::Deref for HeaderValuesAsserter {
    type Target = dyn Fn(HeaderKey, HeaderValues);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
