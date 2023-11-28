use serde::{de::DeserializeOwned, Serialize};

use crate::{accessor::BodyAccessor, body::fallible::FallibleAsserhttpBody};

/// For assertions on http response body
pub trait AsserhttpBody<T> {
    /// Allows verifying json body in a closure
    /// * `asserter` - closure to verify json body
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// # use serde::{Serialize, Deserialize};
    /// use asserhttp::*;
    /// use serde_json::{json, Value};
    ///
    /// #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    /// struct MyStruct { a: String }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     reqwest::get("http://localhost").await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///
    ///     isahc::get("http://localhost").expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     isahc::get_async("http://localhost").await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///
    ///     surf::get("http://localhost").await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///
    ///     // or use your structs
    ///     isahc::get("http://localhost").expect_body_json(|b: MyStruct| assert_eq!(b, MyStruct { a: String::from("b") }));
    /// }
    /// ```
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut T
    where
        B: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug + Unpin,
        F: FnOnce(B);

    /// Expects response body to be json and equal
    /// * `body` - expected json body
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_body_json_eq(json!({"a": "b"}));
    ///     reqwest::get("http://localhost").await.expect_body_json_eq(json!({"a": "b"}));
    ///
    ///     isahc::get("http://localhost").expect_body_json_eq(json!({"a": "b"}));
    ///     isahc::get_async("http://localhost").await.expect_body_json_eq(json!({"a": "b"}));
    ///
    ///     surf::get("http://localhost").await.expect_body_json_eq(json!({"a": "b"}));
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_body_json_eq(json!({"a": "b"}));
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_json_eq(json!({"a": "b"}));
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_body_json_eq(json!({"a": "b"}));
    /// }
    /// ```
    fn expect_body_json_eq<B>(&mut self, body: B) -> &mut T
    where
        B: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug + Unpin + Sized;

    /// Allows verifying text body in a closure
    /// * `asserter` - closure to verify text body
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     reqwest::get("http://localhost").await.expect_body_text(|b| assert_eq!(b, "abcd"));
    ///
    ///     isahc::get("http://localhost").expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     isahc::get_async("http://localhost").await.expect_body_text(|b| assert_eq!(b, "abcd"));
    ///
    ///     surf::get("http://localhost").await.expect_body_text(|b| assert_eq!(b, "abcd"));
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_body_text(|b| assert_eq!(b, "abcd"));
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_text(|b| assert_eq!(b, "abcd"));
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_body_text(|b| assert_eq!(b, "abcd"));
    /// }
    /// ```
    fn expect_body_text<F>(&mut self, asserter: F) -> &mut T
    where
        F: FnOnce(String);

    /// Expects response body to be text and equal
    /// * `body` - expected text body
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_body_text_eq("abcd");
    ///     reqwest::get("http://localhost").await.expect_body_text_eq("abcd");
    ///
    ///     isahc::get("http://localhost").expect_body_text_eq("abcd");
    ///     isahc::get_async("http://localhost").await.expect_body_text_eq("abcd");
    ///
    ///     surf::get("http://localhost").await.expect_body_text_eq("abcd");
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_body_text_eq("abcd");
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_text_eq("abcd");
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_body_text_eq("abcd");
    /// }
    /// ```
    fn expect_body_text_eq<B>(&mut self, body: B) -> &mut T
    where
        B: Into<String>;

    /// Expects response body to be text and to match provided regex
    /// * `regex` - must match text response body
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_body_text_matches("[a-z]+");
    ///     reqwest::get("http://localhost").await.expect_body_text_matches("[a-z]+");
    ///
    ///     isahc::get("http://localhost").expect_body_text_matches("[a-z]+");
    ///     isahc::get_async("http://localhost").await.expect_body_text_matches("[a-z]+");
    ///
    ///     surf::get("http://localhost").await.expect_body_text_matches("[a-z]+");
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_body_text_matches("[a-z]+");
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_text_matches("[a-z]+");
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_body_text_matches("[a-z]+");
    /// }
    /// ```
    fn expect_body_text_matches<R>(&mut self, regex: R) -> &mut T
    where
        R: TryInto<regex::Regex, Error = regex::Error>;

    /// Allows verifying response body bytes in a closure
    /// * `asserter` - closure to verify response body as bytes
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     reqwest::get("http://localhost").await.expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///
    ///     isahc::get("http://localhost").expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     isahc::get("http://localhost").expect_body_bytes(|b| assert_eq!(b, &[97, 98, 99, 100]));
    ///     isahc::get("http://localhost").expect_body_bytes(|b| assert_eq!(b, &vec![97, 98, 99, 100]));
    ///
    ///     // on any client
    ///     isahc::get("http://localhost").expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     isahc::get_async("http://localhost").await.expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///
    ///     surf::get("http://localhost").await.expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    /// }
    /// ```
    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut T
    where
        F: FnOnce(&[u8]);

    /// Expects response body to be equal by comparing bytes
    /// * `body` - expected bytes response body
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_body_bytes_eq(b"abcd");
    ///     reqwest::get("http://localhost").await.expect_body_bytes_eq(b"abcd");
    ///
    ///     isahc::get("http://localhost").expect_body_bytes_eq(b"abcd");
    ///     isahc::get("http://localhost").expect_body_bytes_eq(&[97, 98, 99, 100]);
    ///     isahc::get("http://localhost").expect_body_bytes_eq(&vec![97, 98, 99, 100]);
    ///
    ///     // on any client
    ///     isahc::get("http://localhost").expect_body_bytes_eq(b"abcd");
    ///     isahc::get_async("http://localhost").await.expect_body_bytes_eq(b"abcd");
    ///
    ///     surf::get("http://localhost").await.expect_body_bytes_eq(b"abcd");
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_body_bytes_eq(b"abcd");
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_bytes_eq(b"abcd");
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_body_bytes_eq(b"abcd");
    /// }
    /// ```
    fn expect_body_bytes_eq(&mut self, body: &[u8]) -> &mut T;

    /// Expects a response body to be present and not empty
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_body_present();
    ///     reqwest::get("http://localhost").await.expect_body_present();
    ///
    ///     isahc::get("http://localhost").expect_body_present();
    ///     isahc::get_async("http://localhost").await.expect_body_present();
    ///
    ///     surf::get("http://localhost").await.expect_body_present();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_body_present();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_present();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_body_present();
    /// }
    /// ```
    fn expect_body_present(&mut self) -> &mut T;

    /// Expects a response body to be absent or empty
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_body_absent();
    ///     reqwest::get("http://localhost").await.expect_body_absent();
    ///
    ///     isahc::get("http://localhost").expect_body_absent();
    ///     isahc::get_async("http://localhost").await.expect_body_absent();
    ///
    ///     surf::get("http://localhost").await.expect_body_absent();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_body_absent();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_absent();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_body_absent();
    /// }
    /// ```
    fn expect_body_absent(&mut self) -> &mut T;
}

impl<T> AsserhttpBody<T> for T
where
    T: BodyAccessor,
{
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut T
    where
        B: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug + Unpin,
        F: FnOnce(B),
    {
        #[allow(clippy::blocks_in_conditions)]
        match self.try_expect_body_json(|j| {
            asserter(j);
            Ok(())
        }) {
            Err(e) => panic!("{e}"),
            Ok(r) => r,
        }
    }

    fn expect_body_json_eq<B>(&mut self, body: B) -> &mut T
    where
        B: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug + Unpin,
    {
        match self.try_expect_body_json_eq(body) {
            Err(e) => panic!("{e}"),
            Ok(r) => r,
        }
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut T
    where
        F: FnOnce(String),
    {
        #[allow(clippy::blocks_in_conditions)]
        match self.try_expect_body_text(|t| {
            asserter(t);
            Ok(())
        }) {
            Err(e) => panic!("{e}"),
            Ok(r) => r,
        }
    }

    fn expect_body_text_eq<B>(&mut self, body: B) -> &mut T
    where
        B: Into<String>,
    {
        match self.try_expect_body_text_eq(body) {
            Err(e) => panic!("{e}"),
            Ok(r) => r,
        }
    }

    fn expect_body_text_matches<R>(&mut self, regex: R) -> &mut T
    where
        R: TryInto<regex::Regex, Error = regex::Error>,
    {
        let r: regex::Regex = match regex.try_into() {
            Err(e) => panic!("{e}"),
            Ok(r) => r,
        };
        match self.try_expect_body_text_matches(r.to_string()) {
            Err(e) => panic!("{e}"),
            Ok(r) => r,
        }
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut T
    where
        F: FnOnce(&[u8]),
    {
        #[allow(clippy::blocks_in_conditions)]
        match self.try_expect_body_bytes(|b| {
            asserter(b);
            Ok(())
        }) {
            Err(e) => panic!("{e}"),
            Ok(r) => r,
        }
    }

    fn expect_body_bytes_eq(&mut self, body: &[u8]) -> &mut T {
        match self.try_expect_body_bytes_eq(body) {
            Err(e) => panic!("{e}"),
            Ok(r) => r,
        }
    }

    fn expect_body_present(&mut self) -> &mut T {
        match self.try_expect_body_present() {
            Err(e) => panic!("{e}"),
            Ok(r) => r,
        }
    }

    fn expect_body_absent(&mut self) -> &mut T {
        match self.try_expect_body_absent() {
            Err(e) => panic!("{e}"),
            Ok(r) => r,
        }
    }
}

impl<T, E> AsserhttpBody<T> for Result<T, E>
where
    T: BodyAccessor,
    E: std::fmt::Debug,
{
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut T
    where
        B: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug + Unpin,
        F: FnOnce(B),
    {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_json_eq<B>(&mut self, body: B) -> &mut T
    where
        B: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug + Unpin,
    {
        self.as_mut().unwrap().expect_body_json_eq(body)
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut T
    where
        F: FnOnce(String),
    {
        self.as_mut().unwrap().expect_body_text(asserter)
    }

    fn expect_body_text_eq<B>(&mut self, body: B) -> &mut T
    where
        B: Into<String>,
    {
        self.as_mut().unwrap().expect_body_text_eq(body)
    }

    fn expect_body_text_matches<R>(&mut self, regex: R) -> &mut T
    where
        R: TryInto<regex::Regex, Error = regex::Error>,
    {
        self.as_mut().unwrap().expect_body_text_matches(regex)
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut T
    where
        F: FnOnce(&[u8]),
    {
        self.as_mut().unwrap().expect_body_bytes(asserter)
    }

    fn expect_body_bytes_eq(&mut self, body: &[u8]) -> &mut T {
        self.as_mut().unwrap().expect_body_bytes_eq(body)
    }

    fn expect_body_present(&mut self) -> &mut T {
        self.as_mut().unwrap().expect_body_present()
    }

    fn expect_body_absent(&mut self) -> &mut T {
        self.as_mut().unwrap().expect_body_absent()
    }
}
