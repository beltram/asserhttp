//!
//! Allows fluent assertions for various http client responses.
//! Supports [surf](https://github.com/http-rs/surf) and [isahc](https://github.com/sagebind/isahc).
//!
//! It works for blocking or async client methods and for responses wrapped in `Result`.
//!
//! # Example
//!
//! ## surf
//!
//! ```no_run
//! use asserhttp::*;
//!
//! #[async_std::test]
//! async fn sample_test() {
//!     surf::get("http://localhost/api/any").await.unwrap().expect_status_ok();
//!     // no need to call `.unwrap()` directly
//!     surf::get("http://localhost/api/any").await.expect_status_ed(200);
//!     surf::get("http://localhost/api/any").await.expect_status_ok();
//!     surf::get("http://localhost/api/any").await.expect_status_bad_request();
//!     surf::get("http://localhost/api/any").await.expect_status_internal_server_error();
//!     // and many more !
//! }
//! ```
//!
//! ## surf
//!
//! ```no_run
//! use asserhttp::*;
//!
//! #[async_std::test]
//! async fn sample_test() {
//!     isahc::get_async("http://localhost/api/any").await.unwrap().expect_status_ok();
//!     // no need to call `.unwrap()` directly
//!     isahc::get_async("http://localhost/api/any").await.expect_status_ed(200);
//!     isahc::get_async("http://localhost/api/any").await.expect_status_ok();
//!     isahc::get_async("http://localhost/api/any").await.expect_status_bad_request();
//!     isahc::get_async("http://localhost/api/any").await.expect_status_internal_server_error();
//!     // and many more !
//! }
//! ```
//!
use std::fmt::Debug;
use std::str::FromStr;

use regex::Regex;
use serde::de::DeserializeOwned;

use asserter::body::{assert_body_regex, assert_bytes_body, assert_json_body_eq, assert_text_body};

#[cfg(feature = "surf")]
mod assert_surf;
#[cfg(feature = "isahc")]
mod assert_isahc;

mod asserter;

/// For assertions on http response
pub trait Asserhttp<T>: AsserhttpStatus<T> + AsserhttpHeader<T> + AsserhttpBody<T> {}

/// For assertions on http response status
pub trait AsserhttpStatus<T> {
    /// Expects response status to be equal
    /// * `status` - expected status
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_eq(200);
    ///     isahc::get("http://localhost").expect_status_eq(200);
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_eq(200);
    ///     isahc::get_async("http://localhost").await.expect_status_eq(200);
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_eq(200);
    ///     surf::get("http://localhost").await.expect_status_eq(200);
    /// }
    /// ```
    fn expect_status_eq(&mut self, status: u16) -> &mut T;

    /// Expects response status to be in range
    /// * `lower` - lower inclusive bound
    /// * `upper` - upper exclusive bound
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_in_range(200, 400);
    ///     isahc::get("http://localhost").expect_status_in_range(200, 400);
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_in_range(200, 400);
    ///     isahc::get_async("http://localhost").await.expect_status_in_range(200, 400);
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_in_range(200, 400);
    ///     surf::get("http://localhost").await.expect_status_in_range(200, 400);
    /// }
    /// ```
    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut T;

    /// Expects response status to be in 2xx range
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_success();
    ///     isahc::get("http://localhost").expect_status_success();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_success();
    ///     isahc::get_async("http://localhost").await.expect_status_success();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_success();
    ///     surf::get("http://localhost").await.expect_status_success();
    /// }
    /// ```
    fn expect_status_success(&mut self) -> &mut T { self.expect_status_in_range(200, 300) }

    /// Expects response status to be in 3xx range
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_redirection();
    ///     isahc::get("http://localhost").expect_status_redirection();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_redirection();
    ///     isahc::get_async("http://localhost").await.expect_status_redirection();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_redirection();
    ///     surf::get("http://localhost").await.expect_status_redirection();
    /// }
    /// ```
    fn expect_status_redirection(&mut self) -> &mut T { self.expect_status_in_range(300, 400) }

    /// Expects response status to be in 4xx range
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_client_error();
    ///     isahc::get("http://localhost").expect_status_client_error();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_client_error();
    ///     isahc::get_async("http://localhost").await.expect_status_client_error();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_client_error();
    ///     surf::get("http://localhost").await.expect_status_client_error();
    /// }
    /// ```
    fn expect_status_client_error(&mut self) -> &mut T { self.expect_status_in_range(400, 500) }

    /// Expects response status to be in 5xx range
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_server_error();
    ///     isahc::get("http://localhost").expect_status_server_error();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_server_error();
    ///     isahc::get_async("http://localhost").await.expect_status_server_error();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_server_error();
    ///     surf::get("http://localhost").await.expect_status_server_error();
    /// }
    /// ```
    fn expect_status_server_error(&mut self) -> &mut T { self.expect_status_in_range(500, 600) }

    /// Expects response status to be `OK 200`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_ok();
    ///     isahc::get("http://localhost").expect_status_ok();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_ok();
    ///     isahc::get_async("http://localhost").await.expect_status_ok();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_ok();
    ///     surf::get("http://localhost").await.expect_status_ok();
    /// }
    /// ```
    fn expect_status_ok(&mut self) -> &mut T { self.expect_status_eq(200) }

    /// Expects response status to be `Created 201`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_created();
    ///     isahc::get("http://localhost").expect_status_created();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_created();
    ///     isahc::get_async("http://localhost").await.expect_status_created();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_created();
    ///     surf::get("http://localhost").await.expect_status_created();
    /// }
    /// ```
    fn expect_status_created(&mut self) -> &mut T { self.expect_status_eq(201) }

    /// Expects response status to be `Accepted 202`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_accepted();
    ///     isahc::get("http://localhost").expect_status_accepted();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_accepted();
    ///     isahc::get_async("http://localhost").await.expect_status_accepted();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_accepted();
    ///     surf::get("http://localhost").await.expect_status_accepted();
    /// }
    /// ```
    fn expect_status_accepted(&mut self) -> &mut T { self.expect_status_eq(202) }

    /// Expects response status to be `No Content 204`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_no_content();
    ///     isahc::get("http://localhost").expect_status_no_content();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_no_content();
    ///     isahc::get_async("http://localhost").await.expect_status_no_content();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_no_content();
    ///     surf::get("http://localhost").await.expect_status_no_content();
    /// }
    /// ```
    fn expect_status_no_content(&mut self) -> &mut T { self.expect_status_eq(204) }

    /// Expects response status to be `Partial Content 206`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_partial_content();
    ///     isahc::get("http://localhost").expect_status_partial_content();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_partial_content();
    ///     isahc::get_async("http://localhost").await.expect_status_partial_content();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_partial_content();
    ///     surf::get("http://localhost").await.expect_status_partial_content();
    /// }
    /// ```
    fn expect_status_partial_content(&mut self) -> &mut T { self.expect_status_eq(206) }

    /// Expects response status to be `Bad Request 400`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_bad_request();
    ///     isahc::get("http://localhost").expect_status_bad_request();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_bad_request();
    ///     isahc::get_async("http://localhost").await.expect_status_bad_request();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_bad_request();
    ///     surf::get("http://localhost").await.expect_status_bad_request();
    /// }
    /// ```
    fn expect_status_bad_request(&mut self) -> &mut T { self.expect_status_eq(400) }

    /// Expects response status to be `Unauthorized 401`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_unauthorized();
    ///     isahc::get("http://localhost").expect_status_unauthorized();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_unauthorized();
    ///     isahc::get_async("http://localhost").await.expect_status_unauthorized();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_unauthorized();
    ///     surf::get("http://localhost").await.expect_status_unauthorized();
    /// }
    /// ```
    fn expect_status_unauthorized(&mut self) -> &mut T { self.expect_status_eq(401) }

    /// Expects response status to be `Forbidden 403`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_forbidden();
    ///     isahc::get("http://localhost").expect_status_forbidden();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_forbidden();
    ///     isahc::get_async("http://localhost").await.expect_status_forbidden();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_forbidden();
    ///     surf::get("http://localhost").await.expect_status_forbidden();
    /// }
    /// ```
    fn expect_status_forbidden(&mut self) -> &mut T { self.expect_status_eq(403) }

    /// Expects response status to be `Not Found 404`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_not_found();
    ///     isahc::get("http://localhost").expect_status_not_found();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_not_found();
    ///     isahc::get_async("http://localhost").await.expect_status_not_found();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_not_found();
    ///     surf::get("http://localhost").await.expect_status_not_found();
    /// }
    /// ```
    fn expect_status_not_found(&mut self) -> &mut T { self.expect_status_eq(404) }

    /// Expects response status to be `Conflict 409`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_conflict();
    ///     isahc::get("http://localhost").expect_status_conflict();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_conflict();
    ///     isahc::get_async("http://localhost").await.expect_status_conflict();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_conflict();
    ///     surf::get("http://localhost").await.expect_status_conflict();
    /// }
    /// ```
    fn expect_status_conflict(&mut self) -> &mut T { self.expect_status_eq(409) }

    /// Expects response status to be `Gone 410`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_gone();
    ///     isahc::get("http://localhost").expect_status_gone();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_gone();
    ///     isahc::get_async("http://localhost").await.expect_status_gone();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_gone();
    ///     surf::get("http://localhost").await.expect_status_gone();
    /// }
    /// ```
    fn expect_status_gone(&mut self) -> &mut T { self.expect_status_eq(410) }

    /// Expects response status to be `Internal Server Error 500`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_status_internal_server_error();
    ///     isahc::get("http://localhost").expect_status_internal_server_error();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_internal_server_error();
    ///     isahc::get_async("http://localhost").await.expect_status_internal_server_error();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_internal_server_error();
    ///     surf::get("http://localhost").await.expect_status_internal_server_error();
    /// }
    /// ```
    fn expect_status_internal_server_error(&mut self) -> &mut T { self.expect_status_eq(500) }
}

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
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_header("content-type", "application/json");
    ///     isahc::get("http://localhost").expect_header("content-type", "application/json");
    ///     isahc::get_async("http://localhost").await.unwrap().expect_header("content-type", "application/json");
    ///     isahc::get_async("http://localhost").await.expect_header("content-type", "application/json");
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_header("content-type", "application/json");
    ///     surf::get("http://localhost").await.expect_header("content-type", "application/json");
    /// }
    /// ```
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut T;

    /// Expects response header to be present
    /// * `key` - expected present header key
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_header_present("content-type");
    ///     isahc::get("http://localhost").expect_header_present("content-type");
    ///     isahc::get_async("http://localhost").await.unwrap().expect_header_present("content-type");
    ///     isahc::get_async("http://localhost").await.expect_header_present("content-type");
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_header_present("content-type");
    ///     surf::get("http://localhost").await.expect_header_present("content-type");
    /// }
    /// ```
    fn expect_header_present<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut T;

    /// Expects response header to be absent
    /// * `key` - expected absent header key
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_header_absent("content-type");
    ///     isahc::get("http://localhost").expect_header_absent("content-type");
    ///     isahc::get_async("http://localhost").await.unwrap().expect_header_absent("content-type");
    ///     isahc::get_async("http://localhost").await.expect_header_absent("content-type");
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_header_absent("content-type");
    ///     surf::get("http://localhost").await.expect_header_absent("content-type");
    /// }
    /// ```
    fn expect_header_absent<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut T;

    /// Expects response header `Content-Type: application/json`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_content_type_json();
    ///     isahc::get("http://localhost").expect_content_type_json();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_content_type_json();
    ///     isahc::get_async("http://localhost").await.expect_content_type_json();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_content_type_json();
    ///     surf::get("http://localhost").await.expect_content_type_json();
    /// }
    /// ```
    fn expect_content_type_json(&mut self) -> &mut T {
        self.expect_header(Self::CONTENT_TYPE, Self::APPLICATION_JSON)
    }

    /// Expects response header `Content-Type: text/plain`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_content_type_text();
    ///     isahc::get("http://localhost").expect_content_type_text();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_content_type_text();
    ///     isahc::get_async("http://localhost").await.expect_content_type_text();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_content_type_text();
    ///     surf::get("http://localhost").await.expect_content_type_text();
    /// }
    /// ```
    fn expect_content_type_text(&mut self) -> &mut T {
        self.expect_header(Self::CONTENT_TYPE, Self::TEXT_PLAIN)
    }
}

/// For assertions on http response body
pub trait AsserhttpBody<T> {
    /// Allows verifying json body in a closure
    /// * `asserter` - closure to verify json body
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use serde::Deserialize;
    /// use asserhttp::*;
    /// use serde_json::{json, Value};
    ///
    /// #[derive(Deserialize, Debug, Eq, PartialEq)]
    /// struct MyStruct { a: String }
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     isahc::get("http://localhost").expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     isahc::get_async("http://localhost").await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     surf::get("http://localhost").await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///
    ///     // or use your structs
    ///     isahc::get("http://localhost").expect_body_json(|b: MyStruct| assert_eq!(b, MyStruct { a: String::from("b") }));
    /// }
    /// ```
    fn expect_body_json<B, F>(&mut self, asserter: F) -> &mut T
        where B: DeserializeOwned + PartialEq + Debug + Unpin,
              F: FnOnce(B);

    /// Expects response body to be json and equal
    /// * `body` - expected json body
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    /// use serde_json::json;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_body_json_eq(json!({"a": "b"}));
    ///     isahc::get("http://localhost").expect_body_json_eq(json!({"a": "b"}));
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_json_eq(json!({"a": "b"}));
    ///     isahc::get_async("http://localhost").await.expect_body_json_eq(json!({"a": "b"}));
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_json_eq(json!({"a": "b"}));
    ///     surf::get("http://localhost").await.expect_body_json_eq(json!({"a": "b"}));
    /// }
    /// ```
    fn expect_body_json_eq<B>(&mut self, body: B) -> &mut T where B: DeserializeOwned + PartialEq + Debug + Unpin {
        self.expect_body_json(|actual: B| assert_json_body_eq(actual, body))
    }

    /// Allows verifying text body in a closure
    /// * `asserter` - closure to verify text body
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_body_text(|b| assert_eq!(b, String::from("abcd")));
    ///     isahc::get("http://localhost").unwrap().expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     isahc::get("http://localhost").expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     isahc::get_async("http://localhost").await.expect_body_text(|b| assert_eq!(b, "abcd"));
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     surf::get("http://localhost").await.expect_body_text(|b| assert_eq!(b, "abcd"));
    /// }
    /// ```
    fn expect_body_text<F>(&mut self, asserter: F) -> &mut T where F: FnOnce(String);

    /// Expects response body to be text and equal
    /// * `body` - expected text body
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_body_text_eq(String::from("abcd"));
    ///     isahc::get("http://localhost").unwrap().expect_body_text_eq("abcd");
    ///     isahc::get("http://localhost").expect_body_text_eq("abcd");
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_text_eq("abcd");
    ///     isahc::get_async("http://localhost").await.expect_body_text_eq("abcd");
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_text_eq("abcd");
    ///     surf::get("http://localhost").await.expect_body_text_eq("abcd");
    /// }
    /// ```
    fn expect_body_text_eq<B>(&mut self, body: B) -> &mut T where B: Into<String> {
        self.expect_body_text(|actual| assert_text_body(actual, body.into()))
    }

    /// Expects response body to be text and to match provided regex
    /// * `regex` - must match text response body
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").unwrap().expect_body_text_matches(String::from("[a-z]+"));
    ///     isahc::get("http://localhost").unwrap().expect_body_text_matches("[a-z]+");
    ///     isahc::get("http://localhost").expect_body_text_matches("[a-z]+");
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_text_matches("[a-z]+");
    ///     isahc::get_async("http://localhost").await.expect_body_text_matches("[a-z]+");
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_text_matches("[a-z]+");
    ///     surf::get("http://localhost").await.expect_body_text_matches("[a-z]+");
    /// }
    /// ```
    fn expect_body_text_matches<R>(&mut self, regex: R) -> &mut T where R: Into<String> {
        let regex = Regex::from_str(regex.into().as_str())
            .expect("'{}' is not a valid regex");
        self.expect_body_text(|actual| assert_body_regex(actual, regex))
    }

    /// Allows verifying response body bytes in a closure
    /// * `asserter` - closure to verify response body as bytes
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     isahc::get("http://localhost").expect_body_bytes(|b| assert_eq!(b, &[97, 98, 99, 100]));
    ///     isahc::get("http://localhost").expect_body_bytes(|b| assert_eq!(b, &vec![97, 98, 99, 100]));
    ///
    ///     // on any client
    ///     isahc::get("http://localhost").unwrap().expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     isahc::get("http://localhost").expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     isahc::get_async("http://localhost").await.expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     surf::get("http://localhost").await.expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    /// }
    /// ```
    fn expect_body_bytes<F>(&mut self, asserter: F) -> &mut T where F: FnOnce(&[u8]);

    /// Expects response body to be equal by comparing bytes
    /// * `body` - expected bytes response body
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     isahc::get("http://localhost").expect_body_bytes_eq(b"abcd");
    ///     isahc::get("http://localhost").expect_body_bytes_eq(&[97, 98, 99, 100]);
    ///     isahc::get("http://localhost").expect_body_bytes_eq(&vec![97, 98, 99, 100]);
    ///
    ///     // on any client
    ///     isahc::get("http://localhost").unwrap().expect_body_bytes_eq(b"abcd");
    ///     isahc::get("http://localhost").expect_body_bytes_eq(b"abcd");
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_bytes_eq(b"abcd");
    ///     isahc::get_async("http://localhost").await.expect_body_bytes_eq(b"abcd");
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_bytes_eq(b"abcd");
    ///     surf::get("http://localhost").await.expect_body_bytes_eq(b"abcd");
    /// }
    /// ```
    fn expect_body_bytes_eq(&mut self, body: &[u8]) -> &mut T {
        self.expect_body_bytes(|actual| assert_bytes_body(actual, body))
    }
}