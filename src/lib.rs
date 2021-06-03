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

/// For assertions on http response
pub trait Asserhttp<T>: AsserhttpStatus<T> {}

/// For assertions on http response status
pub trait AsserhttpStatus<T> {
    /// Expects response status to be equal
    /// * `status` - expected status
    fn expect_status_eq(&mut self, status: u16) -> &mut T;
    /// Expects response status to be `OK 200`
    fn expect_status_ok(&mut self) -> &mut T { self.expect_status_eq(200) }
    /// Expects response status to be `Created 201`
    fn expect_status_created(&mut self) -> &mut T { self.expect_status_eq(201) }
    /// Expects response status to be `Accepted 202`
    fn expect_status_accepted(&mut self) -> &mut T { self.expect_status_eq(202) }
    /// Expects response status to be `No Content 204`
    fn expect_status_no_content(&mut self) -> &mut T { self.expect_status_eq(204) }
    /// Expects response status to be `Bad Request 400`
    fn expect_status_bad_request(&mut self) -> &mut T { self.expect_status_eq(400) }
    /// Expects response status to be `Unauthorized 401`
    fn expect_status_unauthorized(&mut self) -> &mut T { self.expect_status_eq(401) }
    /// Expects response status to be `Forbidden 403`
    fn expect_status_forbidden(&mut self) -> &mut T { self.expect_status_eq(403) }
    /// Expects response status to be `Not Found 404`
    fn expect_status_not_found(&mut self) -> &mut T { self.expect_status_eq(404) }
    /// Expects response status to be `Conflict 409`
    fn expect_status_conflict(&mut self) -> &mut T { self.expect_status_eq(409) }
    /// Expects response status to be `Gone 410`
    fn expect_status_gone(&mut self) -> &mut T { self.expect_status_eq(410) }
    /// Expects response status to be `Internal Server Error 500`
    fn expect_status_server_error(&mut self) -> &mut T { self.expect_status_eq(500) }
}