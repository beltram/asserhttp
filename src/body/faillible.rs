use crate::accessor::{BodyAccessor, HeaderAccessor};
use crate::{AsserhttpError, AsserhttpResult, FallibleAsserhttpHeader};
use regex::Regex;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::str::FromStr;

/// For assertions on http response body
pub trait FallibleAsserhttpBody<T> {
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
    fn try_expect_body_json<B, F>(&mut self, asserter: F) -> AsserhttpResult<&mut T>
        where
            B: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug + Unpin,
            F: FnOnce(B) -> AsserhttpResult<()>;

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
    fn try_expect_body_json_eq<B>(&mut self, body: B) -> AsserhttpResult<&mut T>
        where
            B: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug + Unpin,
    {
        self.try_expect_body_json(|actual: B| {
            let actual = serde_json::to_value(&actual)?;
            let expected = serde_json::to_value(&body)?;
            let cfg = assert_json_diff::Config::new(assert_json_diff::CompareMode::Strict);
            assert_json_diff::assert_json_matches_no_panic(&actual, &expected, cfg).map_err(|e| AsserhttpError::JsonBodyMismatch(e))?;
            Ok(())
        })
    }

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
    fn try_expect_body_text<F>(&mut self, asserter: F) -> AsserhttpResult<&mut T>
        where
            F: FnOnce(String) -> AsserhttpResult<()>;

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
    fn try_expect_body_text_eq<B>(&mut self, body: B) -> AsserhttpResult<&mut T>
        where
            B: Into<String>,
    {
        /*self.try_expect_body_text(|actual| {
            let expected = body.into();
            assert_eq!(
                actual, expected,
                "expected text body '{actual}' to be equal to '{expected}' but was not"
            );
        })*/
        todo!()
    }

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
    fn try_expect_body_text_matches<R>(&mut self, regex: R) -> AsserhttpResult<&mut T>
        where
            R: Into<String>,
    {
        /*let regex = Regex::from_str(regex.into().as_str()).expect("'{}' is not a valid regex");
        self.try_expect_body_text(|actual| {
            assert!(
                regex.is_match(actual.as_str()),
                "expected text body '{actual}' to match regex '{regex}' but did not"
            );
        })*/
        todo!()
    }

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
    fn try_expect_body_bytes<F>(&mut self, asserter: F) -> AsserhttpResult<&mut T>
        where
            F: FnOnce(&[u8]) -> AsserhttpResult<()>;

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
    fn try_expect_body_bytes_eq(&mut self, body: &[u8]) -> AsserhttpResult<&mut T> {
        /*self.try_expect_body_bytes(|actual| {
            assert_eq!(actual, body, "expected body '{actual:?}' to be equal to '{body:?}' but was not");
        })*/
        todo!()
    }

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
    fn try_expect_body_present(&mut self) -> AsserhttpResult<&mut T>;

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
    fn try_expect_body_absent(&mut self) -> AsserhttpResult<&mut T>;
}

impl<T> FallibleAsserhttpBody<T> for T
    where
        T: BodyAccessor,
{
    fn try_expect_body_json<B, F>(&mut self, asserter: F) -> AsserhttpResult<&mut T>
        where
            B: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug + Unpin,
            F: FnOnce(B) -> AsserhttpResult<()>,
    {
        asserter(self.get_json()?)?;
        Ok(self)
    }

    fn try_expect_body_text<F>(&mut self, asserter: F) -> AsserhttpResult<&mut T>
        where
            F: FnOnce(String) -> AsserhttpResult<()>,
    {
        asserter(self.get_text()?)?;
        Ok(self)
    }

    fn try_expect_body_bytes<F>(&mut self, asserter: F) -> AsserhttpResult<&mut T>
        where
            F: FnOnce(&[u8]) -> AsserhttpResult<()>,
    {
        asserter(&self.get_bytes()?[..])?;
        Ok(self)
    }

    fn try_expect_body_present(&mut self) -> AsserhttpResult<&mut T> {
        let bytes = &self.get_bytes()?[..];
        if bytes.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(self)
    }

    fn try_expect_body_absent(&mut self) -> AsserhttpResult<&mut T> {
        let bytes = &self.get_bytes()?[..];
        if !bytes.is_empty() {
            return Err(AsserhttpError::BodyPresent);
        }
        Ok(self)
    }
}

impl<T, E> FallibleAsserhttpBody<T> for Result<T, E>
    where
        T: BodyAccessor,
        E: std::fmt::Debug,
{
    fn try_expect_body_json<B, F>(&mut self, asserter: F) -> AsserhttpResult<&mut T>
        where
            B: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug + Unpin,
            F: FnOnce(B) -> AsserhttpResult<()>,
    {
        self.as_mut()
            .map_err(|e| AsserhttpError::HttpError(format!("{e:?}")))?
            .try_expect_body_json(asserter)
    }

    fn try_expect_body_text<F>(&mut self, asserter: F) -> AsserhttpResult<&mut T>
        where
            F: FnOnce(String) -> AsserhttpResult<()>,
    {
        self.as_mut()
            .map_err(|e| AsserhttpError::HttpError(format!("{e:?}")))?
            .try_expect_body_text(asserter)
    }

    fn try_expect_body_bytes<F>(&mut self, asserter: F) -> AsserhttpResult<&mut T>
        where
            F: FnOnce(&[u8]) -> AsserhttpResult<()>,
    {
        self.as_mut()
            .map_err(|e| AsserhttpError::HttpError(format!("{e:?}")))?
            .try_expect_body_bytes(asserter)
    }

    fn try_expect_body_present(&mut self) -> AsserhttpResult<&mut T> {
        self.as_mut()
            .map_err(|e| AsserhttpError::HttpError(format!("{e:?}")))?
            .try_expect_body_present()
    }

    fn try_expect_body_absent(&mut self) -> AsserhttpResult<&mut T> {
        self.as_mut()
            .map_err(|e| AsserhttpError::HttpError(format!("{e:?}")))?
            .try_expect_body_absent()
    }
}
