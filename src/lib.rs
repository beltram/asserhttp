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
#[cfg(feature = "surf")]
mod assert_surf;
#[cfg(feature = "isahc")]
mod assert_isahc;

mod asserter;

/// For assertions on http response
pub trait Asserhttp<T>: AsserhttpStatus<T> + AsserhttpHeader<T> {}

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
}