use serde::{de::DeserializeOwned, Serialize};

use crate::{accessor::BodyAccessor, AsserhttpError, AsserhttpResult};

/// For assertions on http response body returning an error instead of panicking
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
    ///     reqwest::blocking::get("http://localhost").try_expect_body_json(|b: Value| { assert_eq!(b, json!({"a": "b"})); Ok(()) }).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_body_json(|b: Value| { assert_eq!(b, json!({"a": "b"})); Ok(()) }).unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_body_json(|b: Value| { assert_eq!(b, json!({"a": "b"})); Ok(()) }).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_body_json(|b: Value| { assert_eq!(b, json!({"a": "b"})); Ok(()) }).unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_body_json(|b: Value| { assert_eq!(b, json!({"a": "b"})); Ok(()) }).unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_body_json(|b: Value| { assert_eq!(b, json!({"a": "b"})); Ok(()) }).unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_body_json(|b: Value| { assert_eq!(b, json!({"a": "b"})); Ok(()) }).unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_body_json(|b: Value| { assert_eq!(b, json!({"a": "b"})); Ok(()) }).unwrap();
    ///
    ///     // or use your structs
    ///     isahc::get("http://localhost").try_expect_body_json(|b: MyStruct| { assert_eq!(b, MyStruct { a: String::from("b") }); Ok(()) }).unwrap();
    /// }
    /// ```
    /// # Error
    /// * [AsserhttpError::BodyAbsent] when the response body is empty
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
    ///     reqwest::blocking::get("http://localhost").try_expect_body_json_eq(json!({"a": "b"})).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_body_json_eq(json!({"a": "b"})).unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_body_json_eq(json!({"a": "b"})).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_body_json_eq(json!({"a": "b"})).unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_body_json_eq(json!({"a": "b"})).unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_body_json_eq(json!({"a": "b"})).unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_body_json_eq(json!({"a": "b"})).unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_body_json_eq(json!({"a": "b"})).unwrap();
    /// }
    /// ```
    /// # Error
    /// * [AsserhttpError::BodyAbsent] when the response body is empty
    /// * [AsserhttpError::JsonBodyMismatch] when the actual response body is different from the expected one
    fn try_expect_body_json_eq<B>(&mut self, body: B) -> AsserhttpResult<&mut T>
    where
        B: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug + Unpin,
    {
        self.try_expect_body_json(|actual: B| {
            let actual = serde_json::to_value(&actual)?;
            let expected = serde_json::to_value(&body)?;
            let cfg = assert_json_diff::Config::new(assert_json_diff::CompareMode::Strict);
            assert_json_diff::assert_json_matches_no_panic(&actual, &expected, cfg).map_err(AsserhttpError::JsonBodyMismatch)?;
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
    ///     reqwest::blocking::get("http://localhost").try_expect_body_text(|b| { assert_eq!(b, "abcd"); Ok(()) }).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_body_text(|b| { assert_eq!(b, "abcd"); Ok(()) }).unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_body_text(|b| { assert_eq!(b, "abcd"); Ok(()) }).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_body_text(|b| { assert_eq!(b, "abcd"); Ok(()) }).unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_body_text(|b| { assert_eq!(b, "abcd"); Ok(()) }).unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_body_text(|b| { assert_eq!(b, "abcd"); Ok(()) }).unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_body_text(|b| { assert_eq!(b, "abcd"); Ok(()) }).unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_body_text(|b| { assert_eq!(b, "abcd"); Ok(()) }).unwrap();
    /// }
    /// ```
    /// # Error
    /// * [AsserhttpError::BodyAbsent] when the response body is empty
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
    ///     reqwest::blocking::get("http://localhost").try_expect_body_text_eq("abcd").unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_body_text_eq("abcd").unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_body_text_eq("abcd").unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_body_text_eq("abcd").unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_body_text_eq("abcd").unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_body_text_eq("abcd").unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_body_text_eq("abcd").unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_body_text_eq("abcd").unwrap();
    /// }
    /// ```
    /// # Error
    /// * [AsserhttpError::BodyAbsent] when the response body is empty
    /// * [AsserhttpError::TextBodyMismatch] when the actual response body is different from the expected one
    fn try_expect_body_text_eq<B>(&mut self, body: B) -> AsserhttpResult<&mut T>
    where
        B: Into<String>,
    {
        self.try_expect_body_text(|actual| {
            let expected = body.into();
            match actual.as_str() {
                "" => Err(AsserhttpError::BodyAbsent),
                e if e == expected => Ok(()),
                _ => Err(AsserhttpError::TextBodyMismatch { actual, expected }),
            }
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
    ///     reqwest::blocking::get("http://localhost").try_expect_body_text_matches("[a-z]+").unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_body_text_matches("[a-z]+").unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_body_text_matches("[a-z]+").unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_body_text_matches("[a-z]+").unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_body_text_matches("[a-z]+").unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_body_text_matches("[a-z]+").unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_body_text_matches("[a-z]+").unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_body_text_matches("[a-z]+").unwrap();
    /// }
    /// ```
    /// # Error
    /// * [AsserhttpError::BodyAbsent] when the response body is empty
    /// * [AsserhttpError::TextBodyMismatch] when the actual response body is different from the expected one
    /// * [AsserhttpError::RegexError] when the supplied Regex is invalid
    fn try_expect_body_text_matches<R>(&mut self, regex: R) -> AsserhttpResult<&mut T>
    where
        R: TryInto<regex::Regex, Error = regex::Error>,
    {
        self.try_expect_body_text(|actual| {
            let regex = regex.try_into()?;
            match actual.as_str() {
                "" => Err(AsserhttpError::BodyAbsent),
                _ if regex.is_match(actual.as_str()) => Ok(()),
                _ => Err(AsserhttpError::RegexBodyMismatch {
                    actual,
                    regex: regex.to_string(),
                }),
            }
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
    ///     reqwest::blocking::get("http://localhost").try_expect_body_bytes(|b| { assert_eq!(b, b"abcd"); Ok(()) }).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_body_bytes(|b| { assert_eq!(b, b"abcd"); Ok(()) }).unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_body_bytes(|b| { assert_eq!(b, b"abcd"); Ok(()) }).unwrap();
    ///     isahc::get("http://localhost").try_expect_body_bytes(|b| { assert_eq!(b, &[97, 98, 99, 100]); Ok(()) }).unwrap();
    ///     isahc::get("http://localhost").try_expect_body_bytes(|b| { assert_eq!(b, &vec![97, 98, 99, 100]); Ok(()) }).unwrap();
    ///
    ///     // on any client
    ///     isahc::get("http://localhost").try_expect_body_bytes(|b| { assert_eq!(b, b"abcd"); Ok(()) }).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_body_bytes(|b| { assert_eq!(b, b"abcd"); Ok(()) }).unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_body_bytes(|b| { assert_eq!(b, b"abcd"); Ok(()) }).unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_body_bytes(|b| { assert_eq!(b, b"abcd"); Ok(()) }).unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_body_bytes(|b| { assert_eq!(b, b"abcd"); Ok(()) }).unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_body_bytes(|b| { assert_eq!(b, b"abcd"); Ok(()) }).unwrap();
    /// }
    /// ```
    /// # Error
    /// * [AsserhttpError::BodyAbsent] when the response body is empty
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
    /// # Error
    /// * [AsserhttpError::BodyAbsent] when the response body is empty
    /// * [AsserhttpError::BytesBodyMismatch] when the actual response body is different from the expected one
    fn try_expect_body_bytes_eq(&mut self, body: &[u8]) -> AsserhttpResult<&mut T> {
        self.try_expect_body_bytes(|actual| {
            let expected = body;
            match actual {
                b"" => Err(AsserhttpError::BodyAbsent),
                e if e == expected => Ok(()),
                _ => Err(AsserhttpError::BytesBodyMismatch {
                    actual: actual.to_vec(),
                    expected: expected.to_vec(),
                }),
            }
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
    ///     reqwest::blocking::get("http://localhost").try_expect_body_present().unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_body_present().unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_body_present().unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_body_present().unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_body_present().unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_body_present().unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_body_present().unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_body_present().unwrap();
    /// }
    /// ```
    /// # Error
    /// * [AsserhttpError::BodyAbsent] when the response body is empty
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
    ///     reqwest::blocking::get("http://localhost").try_expect_body_absent().unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_body_absent().unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_body_absent().unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_body_absent().unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_body_absent().unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_body_absent().unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_body_absent().unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_body_absent().unwrap();
    /// }
    /// ```
    /// # Error
    /// * [AsserhttpError::BodyPresent] when the response body is present
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
        match self.get_bytes() {
            Err(AsserhttpError::BodyAbsent) => Err(AsserhttpError::BodyAbsent),
            _ => Ok(self),
        }
    }

    fn try_expect_body_absent(&mut self) -> AsserhttpResult<&mut T> {
        match self.get_bytes() {
            Err(AsserhttpError::BodyAbsent) => Ok(self),
            _ => Err(AsserhttpError::BodyPresent),
        }
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
