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
//!     surf::get("http://localhost/api/any").await.expect_status_server_error();
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
//!     isahc::get_async("http://localhost/api/any").await.expect_status_server_error();
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
pub trait Asserhttp<T>: AsserhttpStatus<T> {}

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
    ///     isahc::get("http://localhost").unwrap().expect_status_server_error();
    ///     isahc::get("http://localhost").expect_status_server_error();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_server_error();
    ///     isahc::get_async("http://localhost").await.expect_status_server_error();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_server_error();
    ///     surf::get("http://localhost").await.expect_status_server_error();
    /// }
    /// ```
    fn expect_status_server_error(&mut self) -> &mut T { self.expect_status_eq(500) }
}