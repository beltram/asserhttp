use std::{fmt::Debug, str::FromStr};

use regex::Regex;
use serde::{de::DeserializeOwned, Serialize};

const EMPTY_BODY_JSON_MSG: &str = "expected a json body but no response body was present";
const EMPTY_BODY_TEXT_MSG: &str = "expected a text body but no response body was present";
const EMPTY_BODY_BYTES_MSG: &str = "expected a response body but no response body was present";
const EXPECTED_BODY_PRESENT_MSG: &str = "expected a response body but no response body was present";
const EXPECTED_BODY_ABSENT_MSG: &str = "expected no response body but a response body was present";

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
        B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
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
        B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
    {
        self.expect_body_json(|actual: B| {
            assert_eq!(
                actual,
                body,
                "expected json body '{}' to be equal to '{}' but was not",
                to_json_string(&actual),
                to_json_string(&body)
            );
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
        B: Into<String>,
    {
        self.expect_body_text(|actual| {
            let expected = body.into();
            assert_eq!(
                actual, expected,
                "expected text body '{actual}' to be equal to '{expected}' but was not"
            );
        })
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
    fn expect_body_text_matches<R>(&mut self, regex: R) -> &mut T
    where
        R: Into<String>,
    {
        let regex = Regex::from_str(regex.into().as_str()).expect("'{}' is not a valid regex");
        self.expect_body_text(|actual| {
            assert!(
                regex.is_match(actual.as_str()),
                "expected text body '{actual}' to match regex '{regex}' but did not"
            );
        })
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
    fn expect_body_bytes_eq(&mut self, body: &[u8]) -> &mut T {
        self.expect_body_bytes(|actual| {
            assert_eq!(actual, body, "expected body '{actual:?}' to be equal to '{body:?}' but was not");
        })
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

fn to_json_string<B>(json: &B) -> String
where
    B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
{
    serde_json::to_string(json).unwrap_or_else(|_| String::from("Failed serializing actual response body"))
}

impl<T> AsserhttpBody<T> for T
where
    T: super::accessor::BodyAccessor,
{
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut T
    where
        B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
        F: FnOnce(B),
    {
        if let Ok(actual) = self.get_json() {
            asserter(actual)
        } else {
            std::panic::panic_any(EMPTY_BODY_JSON_MSG)
        }
        self
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut T
    where
        F: FnOnce(String),
    {
        if let Ok(actual) = self.get_text() {
            if !actual.is_empty() {
                asserter(actual)
            } else {
                std::panic::panic_any(EMPTY_BODY_TEXT_MSG)
            }
        } else {
            std::panic::panic_any(EMPTY_BODY_TEXT_MSG)
        }
        self
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut T
    where
        F: FnOnce(&[u8]),
    {
        if let Ok(actual) = self.get_bytes() {
            if !actual.is_empty() {
                asserter(actual.as_slice())
            } else {
                std::panic::panic_any(EMPTY_BODY_BYTES_MSG)
            }
        } else {
            std::panic::panic_any(EMPTY_BODY_BYTES_MSG)
        }
        self
    }

    fn expect_body_present(&mut self) -> &mut T {
        if let Ok(actual) = self.get_bytes() {
            assert!(!actual.is_empty(), "{}", EXPECTED_BODY_PRESENT_MSG);
        } else {
            std::panic::panic_any(EXPECTED_BODY_PRESENT_MSG)
        }
        self
    }

    fn expect_body_absent(&mut self) -> &mut T {
        if let Ok(actual) = self.get_bytes() {
            assert!(actual.is_empty(), "{}", EXPECTED_BODY_ABSENT_MSG);
        }
        self
    }
}

impl<T, E> AsserhttpBody<T> for Result<T, E>
where
    T: super::accessor::BodyAccessor,
    E: Debug,
{
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut T
    where
        B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin,
        F: FnOnce(B),
    {
        self.as_mut().unwrap().expect_body_json(asserter)
    }

    fn expect_body_text<F>(&mut self, asserter: F) -> &mut T
    where
        F: FnOnce(String),
    {
        self.as_mut().unwrap().expect_body_text(asserter)
    }

    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut T
    where
        F: FnOnce(&[u8]),
    {
        self.as_mut().unwrap().expect_body_bytes(asserter)
    }

    fn expect_body_present(&mut self) -> &mut T {
        self.as_mut().unwrap().expect_body_present()
    }

    fn expect_body_absent(&mut self) -> &mut T {
        self.as_mut().unwrap().expect_body_absent()
    }
}
