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
//!     surf::get("http://localhost/api/any").await.unwrap().assert_status_ok();
//!     // no need to call `.unwrap()` directly
//!     surf::get("http://localhost/api/any").await.assert_status_ed(200);
//!     surf::get("http://localhost/api/any").await.assert_status_ok();
//!     surf::get("http://localhost/api/any").await.assert_status_bad_request();
//!     surf::get("http://localhost/api/any").await.assert_status_server_error();
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
//!     isahc::get_async("http://localhost/api/any").await.unwrap().assert_status_ok();
//!     // no need to call `.unwrap()` directly
//!     isahc::get_async("http://localhost/api/any").await.assert_status_ed(200);
//!     isahc::get_async("http://localhost/api/any").await.assert_status_ok();
//!     isahc::get_async("http://localhost/api/any").await.assert_status_bad_request();
//!     isahc::get_async("http://localhost/api/any").await.assert_status_server_error();
//!     // and many more !
//! }
//! ```
//!
mod assert_surf;
mod assert_isahc;

/// For assertions on http response
pub trait Asserhttp: AsserhttpStatus {}

/// For assertions on http response status
pub trait AsserhttpStatus {
    /// Expects response status to be equal
    /// * `status` - expected status
    fn assert_status_eq(&mut self, status: u16) -> &mut Self;
    /// Expects response status to be `OK 200`
    fn assert_status_ok(&mut self) -> &mut Self { self.assert_status_eq(200) }
    /// Expects response status to be `Created 201`
    fn assert_status_created(&mut self) -> &mut Self { self.assert_status_eq(201) }
    /// Expects response status to be `Accepted 202`
    fn assert_status_accepted(&mut self) -> &mut Self { self.assert_status_eq(202) }
    /// Expects response status to be `No Content 204`
    fn assert_status_no_content(&mut self) -> &mut Self { self.assert_status_eq(204) }
    /// Expects response status to be `Bad Request 400`
    fn assert_status_bad_request(&mut self) -> &mut Self { self.assert_status_eq(400) }
    /// Expects response status to be `Unauthorized 401`
    fn assert_status_unauthorized(&mut self) -> &mut Self { self.assert_status_eq(401) }
    /// Expects response status to be `Forbidden 403`
    fn assert_status_forbidden(&mut self) -> &mut Self { self.assert_status_eq(403) }
    /// Expects response status to be `Not Found 404`
    fn assert_status_not_found(&mut self) -> &mut Self { self.assert_status_eq(404) }
    /// Expects response status to be `Conflict 409`
    fn assert_status_conflict(&mut self) -> &mut Self { self.assert_status_eq(409) }
    /// Expects response status to be `Gone 410`
    fn assert_status_gone(&mut self) -> &mut Self { self.assert_status_eq(410) }
    /// Expects response status to be `Internal Server Error 500`
    fn assert_status_server_error(&mut self) -> &mut Self { self.assert_status_eq(500) }
}

/// For assertions on http response unwrapping result
pub trait TryAsserhttp<T>: TryAsserhttpStatus<T> {}

/// For assertions on http response status unwrapping result
pub trait TryAsserhttpStatus<T> {
    /// Expects response status to be equal
    /// * `status` - expected status
    fn assert_status_eq(&mut self, status: u16) -> &mut T;
    /// Expects response status to be `OK 200`
    fn assert_status_ok(&mut self) -> &mut T { self.assert_status_eq(200) }
    /// Expects response status to be `Created 201`
    fn assert_status_created(&mut self) -> &mut T { self.assert_status_eq(201) }
    /// Expects response status to be `Accepted 202`
    fn assert_status_accepted(&mut self) -> &mut T { self.assert_status_eq(202) }
    /// Expects response status to be `No Content 204`
    fn assert_status_no_content(&mut self) -> &mut T { self.assert_status_eq(204) }
    /// Expects response status to be `Bad Request 400`
    fn assert_status_bad_request(&mut self) -> &mut T { self.assert_status_eq(400) }
    /// Expects response status to be `Unauthorized 401`
    fn assert_status_unauthorized(&mut self) -> &mut T { self.assert_status_eq(401) }
    /// Expects response status to be `Forbidden 403`
    fn assert_status_forbidden(&mut self) -> &mut T { self.assert_status_eq(403) }
    /// Expects response status to be `Not Found 404`
    fn assert_status_not_found(&mut self) -> &mut T { self.assert_status_eq(404) }
    /// Expects response status to be `Conflict 409`
    fn assert_status_conflict(&mut self) -> &mut T { self.assert_status_eq(409) }
    /// Expects response status to be `Gone 410`
    fn assert_status_gone(&mut self) -> &mut T { self.assert_status_eq(410) }
    /// Expects response status to be `Internal Server Error 500`
    fn assert_status_server_error(&mut self) -> &mut T { self.assert_status_eq(500) }
}