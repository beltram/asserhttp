use super::accessor;

/// For assertions on http response headers
pub trait AsserhttpHeader<T> {
    const CONTENT_TYPE: &'static str = "content-type";
    const APPLICATION_JSON: &'static str = "application/json";
    const TEXT_PLAIN: &'static str = "text/plain";

    /// Expects response header to be equal
    /// * `key` - expected header key
    /// * `value` - expected header value
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_header("content-type", "application/json");
    ///     reqwest::blocking::get("http://localhost").expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///     reqwest::get("http://localhost").await.expect_header("content-type", "application/json");
    ///     reqwest::get("http://localhost").await.expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///
    ///     isahc::get("http://localhost").expect_header("content-type", "application/json");
    ///     isahc::get("http://localhost").expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///     isahc::get_async("http://localhost").await.expect_header("content-type", "application/json");
    ///     isahc::get_async("http://localhost").await.expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///
    ///     surf::get("http://localhost").await.expect_header("content-type", "application/json");
    ///     surf::get("http://localhost").await.expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_header("content-type", "application/json");
    ///     ureq::get("http://localhost").call().or_any_status().expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header("content-type", "application/json");
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_header("content-type", "application/json");
    ///     awc::Client::default().get("http://localhost").send().await.expect_header("content-type", |h: &str| assert!(h.contains("application/")));
    /// }
    /// ```
    fn expect_header(&mut self, key: impl AsRef<str>, value: impl Into<HeaderValueAsserter>) -> &mut T;

    /// Expects response multi valued headers to be equal
    /// * `key` - expected header key
    /// * `value` - expected header values
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     reqwest::get("http://localhost").await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///
    ///     isahc::get("http://localhost").expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     isahc::get_async("http://localhost").await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///
    ///     surf::get("http://localhost").await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_headers("cache-control", ["no-cache", "no-store"]);
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_headers("cache-control", ["no-cache", "no-store"]);
    /// }
    /// ```
    fn expect_headers<'a>(&mut self, key: impl AsRef<str>, value: impl Into<Vec<&'a str>>) -> &mut T;

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
    ///     reqwest::get("http://localhost").await.expect_header_present("content-type");
    ///
    ///     isahc::get("http://localhost").expect_header_present("content-type");
    ///     isahc::get_async("http://localhost").await.expect_header_present("content-type");
    ///
    ///     surf::get("http://localhost").await.expect_header_present("content-type");
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_header_present("content-type");
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header_present("content-type");
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_header_present("content-type");
    /// }
    /// ```
    fn expect_header_present(&mut self, key: impl AsRef<str>) -> &mut T;

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
    ///     reqwest::get("http://localhost").await.expect_header_absent("content-type");
    ///
    ///     isahc::get("http://localhost").expect_header_absent("content-type");
    ///     isahc::get_async("http://localhost").await.expect_header_absent("content-type");
    ///
    ///     surf::get("http://localhost").await.expect_header_absent("content-type");
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_header_absent("content-type");
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header_absent("content-type");
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_header_absent("content-type");
    /// }
    /// ```
    fn expect_header_absent(&mut self, key: impl AsRef<str>) -> &mut T;

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
        self.expect_header(Self::CONTENT_TYPE, Self::APPLICATION_JSON)
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
        self.expect_header(Self::CONTENT_TYPE, Self::TEXT_PLAIN)
    }
}

impl<T> AsserhttpHeader<T> for T where T: accessor::HeaderAccessor {
    fn expect_header(&mut self, key: impl AsRef<str>, value: impl Into<HeaderValueAsserter>) -> &mut T {
        assert_header_key(self.get_keys(), key.as_ref());
        let actual_values = self.get_values(key.as_ref());
        assert_eq!(actual_values.len(), 1,
                   "expected header '{}' to be single valued. Had '{}' values '{:?}'. Use 'expect_headers' instead.",
                   key.as_ref(), actual_values.len(), actual_values);
        value.into().0(key.as_ref().to_string(), actual_values.first().unwrap().to_string());
        self
    }

    fn expect_headers<'a>(&mut self, key: impl AsRef<str>, value: impl Into<Vec<&'a str>>) -> &mut T {
        assert_header_key(self.get_keys(), key.as_ref());
        let actual_values = self.get_values(key.as_ref());
        let expected = value.into().iter().map(|v| v.to_string()).collect::<Vec<_>>();
        assert!(!expected.is_empty(), "no value expected for header '{}'. Use 'expect_header_present' instead", key.as_ref());
        let same_size = actual_values.len() == expected.len();
        let all_eq = actual_values.iter()
            .zip(expected.iter())
            .all(|(a, b)| a == b);
        assert!(same_size && all_eq, "expected header '{}' to contain values '{:?}' but contained '{:?}'", key.as_ref(), expected, actual_values);
        self
    }

    fn expect_header_present(&mut self, key: impl AsRef<str>) -> &mut T {
        assert_header_key(self.get_keys(), key.as_ref());
        self
    }

    fn expect_header_absent(&mut self, key: impl AsRef<str>) -> &mut T {
        assert!(!self.get_keys().into_iter().any(|k| k.eq_ignore_ascii_case(key.as_ref())),
                "expected no header named '{}' but one found", key.as_ref());
        self
    }
}

pub fn assert_header_key(actual_keys: Vec<String>, expected: &str) {
    assert!(actual_keys.into_iter().any(|k| k.eq_ignore_ascii_case(expected)),
            "expected one header named '{}' but none found", expected);
}

impl<T, E> AsserhttpHeader<T> for Result<T, E> where
    T: accessor::HeaderAccessor,
    E: std::fmt::Debug {
    fn expect_header(&mut self, key: impl AsRef<str>, value: impl Into<HeaderValueAsserter>) -> &mut T {
        self.as_mut().unwrap().expect_header(key, value)
    }

    fn expect_headers<'a>(&mut self, key: impl AsRef<str>, value: impl Into<Vec<&'a str>>) -> &mut T {
        self.as_mut().unwrap().expect_headers(key, value)
    }

    fn expect_header_present(&mut self, key: impl AsRef<str>) -> &mut T {
        self.as_mut().unwrap().expect_header_present(key)
    }

    fn expect_header_absent(&mut self, key: impl AsRef<str>) -> &mut T {
        self.as_mut().unwrap().expect_header_absent(key)
    }
}

pub struct HeaderValueAsserter(Box<dyn Fn(String, String)>);

impl From<&'static String> for HeaderValueAsserter {
    fn from(expected: &'static String) -> Self {
        Self::from(expected.as_str())
    }
}

impl From<&'static str> for HeaderValueAsserter {
    fn from(expected: &'static str) -> Self {
        Self(Box::new(move |key, value| {
            assert_eq!(&value, expected, "expected header '{}' to be equal to '{}' but was '{}'", key, expected, value)
        }))
    }
}

impl<F: 'static> From<F> for HeaderValueAsserter where F: Fn(&'static str) {
    fn from(fun: F) -> Self {
        Self(Box::new(move |_, value| fun(Box::leak(Box::new(value)))))
    }
}