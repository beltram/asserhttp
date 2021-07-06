//!
//! Allows fluent assertions for various http client responses.
//! Supports [reqwest](https://github.com/seanmonstar/reqwest), [hyper](https://github.com/hyperium/hyper),
//! [awc](https://docs.rs/awc) (Actix Web Client),
//! [surf](https://github.com/http-rs/surf) and [isahc](https://github.com/sagebind/isahc).
//!
//! It works for blocking or async client methods and for responses wrapped in `Result`.
//!
//! # API
//!
//! Here's the list of all the provided asserters.
//!
//! ```no_run
//! # use isahc;
//! # use serde::Deserialize;
//! # use serde_json::{json, Value};
//! use asserhttp::*;
//!
//! #[test]
//! fn my_test() {
//! #    isahc::get("http://localhost/api/any")
//!     // status
//!     .expect_status_eq(200)
//!     .expect_status_in_range(200, 400)
//!     .expect_status_success()
//!     .expect_status_redirection()
//!     .expect_status_client_error()
//!     .expect_status_server_error()
//!     .expect_status_ok()
//!     .expect_status_created()
//!     .expect_status_accepted()
//!     .expect_status_no_content()
//!     .expect_status_partial_content()
//!     .expect_status_bad_request()
//!     .expect_status_unauthorized()
//!     .expect_status_forbidden()
//!     .expect_status_not_found()
//!     .expect_status_conflict()
//!     .expect_status_gone()
//!     .expect_status_internal_server_error()
//!     // header
//!     .expect_header("content-type", "application/pdf")
//!     .expect_headers("cache-control", ["no-cache", "no-store"])
//!     .expect_header_present("x-my-header")
//!     .expect_header_absent("x-my-header")
//!     .expect_content_type_json()
//!     .expect_content_type_text()
//!     // body
//!     .expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})))
//!     .expect_body_json_eq(json!({"name": "jdoe"}))
//!     .expect_body_text(|b| assert_eq!(b, "abcd"))
//!     .expect_body_text_eq("abcd")
//!     .expect_body_text_matches("[a-z]+")
//!     .expect_body_bytes(|b| assert_eq!(b, b"abcd"))
//!     .expect_body_bytes_eq(b"abcd")
//!     .expect_body_present()
//!     .expect_body_absent();
//! }
//! ```
//!
//! # Example
//!
//! ## reqwest
//!
//! ```no_run
//! use reqwest;
//! use asserhttp::*;
//!
//! #[tokio::test]
//! async fn sample_test() {
//!     reqwest::get("http://localhost/api/any").await.unwrap().expect_status_ok();
//!     // no need to call `.unwrap()` directly
//!     reqwest::get("http://localhost/api/any").await.expect_status_eq(200);
//!     reqwest::get("http://localhost/api/any").await.expect_status_ok();
//!     reqwest::get("http://localhost/api/any").await.expect_status_bad_request();
//!     reqwest::get("http://localhost/api/any").await.expect_status_internal_server_error();
//!     // chain expectations
//!     reqwest::get("http://localhost/api/any").await
//!         .expect_status_ok()
//!         .expect_content_type_json()
//!         .expect_body_json_eq(json!({"name": "jdoe"}));
//!     // and many more !
//! }
//! ```
//!
//! ## hyper
//!
//! ```no_run
//! use hyper::Client;
//! use asserhttp::*;
//!
//! #[tokio::test]
//! async fn sample_test() {
//!     Client::new().get("http://localhost/api/any").await.unwrap().expect_status_ok();
//!     // no need to call `.unwrap()` directly
//!     Client::new().get("http://localhost/api/any").await.expect_status_eq(200);
//!     Client::new().get("http://localhost/api/any").await.expect_status_ok();
//!     Client::new().get("http://localhost/api/any").await.expect_status_bad_request();
//!     Client::new().get("http://localhost/api/any").await.expect_status_internal_server_error();
//!     // chain expectations
//!     Client::new().get("http://localhost/api/any").await
//!         .expect_status_ok()
//!         .expect_content_type_json()
//!         .expect_body_json_eq(json!({"name": "jdoe"}));
//!     // and many more !
//! }
//! ```
//!
//! ## awc (Actix Web Client)
//!
//! ```no_run
//! use actix_rt::System;
//! use awc::Client;
//! use asserhttp::*;
//!
//! #[test]
//! fn sample_test() {
//!     System::new("test").block_on(async move {
//!         Client::default().get("http://localhost/api/any").await.unwrap().expect_status_ok();
//!         // no need to call `.unwrap()` directly
//!         Client::default().get("http://localhost/api/any").await.expect_status_eq(200);
//!         Client::default().get("http://localhost/api/any").await.expect_status_ok();
//!         Client::default().get("http://localhost/api/any").await.expect_status_bad_request();
//!         Client::default().get("http://localhost/api/any").await.expect_status_internal_server_error();
//!         // chain expectations
//!         Client::default().get("http://localhost/api/any").await
//!             .expect_status_ok()
//!             .expect_content_type_json()
//!             .expect_body_json_eq(json!({"name": "jdoe"}));
//!         // and many more !
//!     });
//! }
//! ```
//!
//! ## surf
//!
//! ```no_run
//! use surf;
//! use asserhttp::*;
//!
//! #[async_std::test]
//! async fn sample_test() {
//!     surf::get("http://localhost/api/any").await.unwrap().expect_status_ok();
//!     // no need to call `.unwrap()` directly
//!     surf::get("http://localhost/api/any").await.expect_status_eq(200);
//!     surf::get("http://localhost/api/any").await.expect_status_ok();
//!     surf::get("http://localhost/api/any").await.expect_status_bad_request();
//!     surf::get("http://localhost/api/any").await.expect_status_internal_server_error();
//!     // chain expectations
//!     surf::get("http://localhost/api/any").await
//!         .expect_status_ok()
//!         .expect_content_type_json()
//!         .expect_body_json_eq(json!({"name": "jdoe"}));
//!     // and many more !
//! }
//! ```
//!
//! ## isahc
//!
//! ```no_run
//! use isahc;
//! use asserhttp::*;
//!
//! #[async_std::test]
//! async fn sample_test() {
//!     isahc::get_async("http://localhost/api/any").await.unwrap().expect_status_ok();
//!     // no need to call `.unwrap()` directly
//!     isahc::get_async("http://localhost/api/any").await.expect_status_eq(200);
//!     isahc::get_async("http://localhost/api/any").await.expect_status_ok();
//!     isahc::get_async("http://localhost/api/any").await.expect_status_bad_request();
//!     isahc::get_async("http://localhost/api/any").await.expect_status_internal_server_error();
//!     // chain expectations
//!     isahc::get_async("http://localhost/api/any").await
//!         .expect_status_ok()
//!         .expect_content_type_json()
//!         .expect_body_json_eq(json!({"name": "jdoe"}));
//!     // and many more !
//! }
//! ```
//!
use std::{fmt::Debug, str::FromStr};

use regex::Regex;
use serde::de::DeserializeOwned;

use asserter::body::{assert_body_regex, assert_bytes_body, assert_json_body_eq, assert_text_body};

#[cfg(feature = "surf")]
mod assert_surf;
#[cfg(feature = "isahc")]
mod assert_isahc;
#[cfg(feature = "reqwest")]
mod assert_reqwest;
#[cfg(feature = "hyper")]
mod assert_hyper;
#[cfg(feature = "actix")]
mod assert_awc;

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
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_eq(200);
    ///     reqwest::blocking::get("http://localhost").expect_status_eq(200);
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_eq(200);
    ///     reqwest::get("http://localhost").await.expect_status_eq(200);
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_eq(200);
    ///     isahc::get("http://localhost").expect_status_eq(200);
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_eq(200);
    ///     isahc::get_async("http://localhost").await.expect_status_eq(200);
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_eq(200);
    ///     surf::get("http://localhost").await.expect_status_eq(200);
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_eq(200);
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_eq(200);
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_eq(200);
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_eq(200);
    ///     });
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
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_in_range(200, 400);
    ///     reqwest::blocking::get("http://localhost").expect_status_in_range(200, 400);
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_in_range(200, 400);
    ///     reqwest::get("http://localhost").await.expect_status_in_range(200, 400);
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_in_range(200, 400);
    ///     isahc::get("http://localhost").expect_status_in_range(200, 400);
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_in_range(200, 400);
    ///     isahc::get_async("http://localhost").await.expect_status_in_range(200, 400);
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_in_range(200, 400);
    ///     surf::get("http://localhost").await.expect_status_in_range(200, 400);
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_in_range(200, 400);
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_in_range(200, 400);
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_in_range(200, 400);
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_in_range(200, 400);
    ///     });
    /// }
    /// ```
    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut T;

    /// Expects response status to be in 2xx range
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_success();
    ///     reqwest::blocking::get("http://localhost").expect_status_success();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_success();
    ///     reqwest::get("http://localhost").await.expect_status_success();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_success();
    ///     isahc::get("http://localhost").expect_status_success();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_success();
    ///     isahc::get_async("http://localhost").await.expect_status_success();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_success();
    ///     surf::get("http://localhost").await.expect_status_success();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_success();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_success();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_success();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_success();
    ///     });
    /// }
    /// ```
    fn expect_status_success(&mut self) -> &mut T { self.expect_status_in_range(200, 300) }

    /// Expects response status to be in 3xx range
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_redirection();
    ///     reqwest::blocking::get("http://localhost").expect_status_redirection();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_redirection();
    ///     reqwest::get("http://localhost").await.expect_status_redirection();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_redirection();
    ///     isahc::get("http://localhost").expect_status_redirection();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_redirection();
    ///     isahc::get_async("http://localhost").await.expect_status_redirection();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_redirection();
    ///     surf::get("http://localhost").await.expect_status_redirection();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_redirection();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_redirection();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_redirection();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_redirection();
    ///     });
    /// }
    /// ```
    fn expect_status_redirection(&mut self) -> &mut T { self.expect_status_in_range(300, 400) }

    /// Expects response status to be in 4xx range
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_client_error();
    ///     reqwest::blocking::get("http://localhost").expect_status_client_error();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_client_error();
    ///     reqwest::get("http://localhost").await.expect_status_client_error();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_client_error();
    ///     isahc::get("http://localhost").expect_status_client_error();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_client_error();
    ///     isahc::get_async("http://localhost").await.expect_status_client_error();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_client_error();
    ///     surf::get("http://localhost").await.expect_status_client_error();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_client_error();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_client_error();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_client_error();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_client_error();
    ///     });
    /// }
    /// ```
    fn expect_status_client_error(&mut self) -> &mut T { self.expect_status_in_range(400, 500) }

    /// Expects response status to be in 5xx range
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_server_error();
    ///     reqwest::blocking::get("http://localhost").expect_status_server_error();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_server_error();
    ///     reqwest::get("http://localhost").await.expect_status_server_error();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_server_error();
    ///     isahc::get("http://localhost").expect_status_server_error();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_server_error();
    ///     isahc::get_async("http://localhost").await.expect_status_server_error();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_server_error();
    ///     surf::get("http://localhost").await.expect_status_server_error();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_server_error();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_server_error();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_server_error();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_server_error();
    ///     });
    /// }
    /// ```
    fn expect_status_server_error(&mut self) -> &mut T { self.expect_status_in_range(500, 600) }

    /// Expects response status to be `OK 200`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_ok();
    ///     reqwest::blocking::get("http://localhost").expect_status_ok();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_ok();
    ///     reqwest::get("http://localhost").await.expect_status_ok();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_ok();
    ///     isahc::get("http://localhost").expect_status_ok();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_ok();
    ///     isahc::get_async("http://localhost").await.expect_status_ok();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_ok();
    ///     surf::get("http://localhost").await.expect_status_ok();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_ok();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_ok();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_ok();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_ok();
    ///     });
    /// }
    /// ```
    fn expect_status_ok(&mut self) -> &mut T { self.expect_status_eq(200) }

    /// Expects response status to be `Created 201`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_created();
    ///     reqwest::blocking::get("http://localhost").expect_status_created();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_created();
    ///     reqwest::get("http://localhost").await.expect_status_created();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_created();
    ///     isahc::get("http://localhost").expect_status_created();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_created();
    ///     isahc::get_async("http://localhost").await.expect_status_created();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_created();
    ///     surf::get("http://localhost").await.expect_status_created();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_created();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_created();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_created();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_created();
    ///     });
    /// }
    /// ```
    fn expect_status_created(&mut self) -> &mut T { self.expect_status_eq(201) }

    /// Expects response status to be `Accepted 202`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_accepted();
    ///     reqwest::blocking::get("http://localhost").expect_status_accepted();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_accepted();
    ///     reqwest::get("http://localhost").await.expect_status_accepted();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_accepted();
    ///     isahc::get("http://localhost").expect_status_accepted();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_accepted();
    ///     isahc::get_async("http://localhost").await.expect_status_accepted();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_accepted();
    ///     surf::get("http://localhost").await.expect_status_accepted();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_accepted();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_accepted();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_accepted();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_accepted();
    ///     });
    /// }
    /// ```
    fn expect_status_accepted(&mut self) -> &mut T { self.expect_status_eq(202) }

    /// Expects response status to be `No Content 204`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_no_content();
    ///     reqwest::blocking::get("http://localhost").expect_status_no_content();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_no_content();
    ///     reqwest::get("http://localhost").await.expect_status_no_content();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_no_content();
    ///     isahc::get("http://localhost").expect_status_no_content();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_no_content();
    ///     isahc::get_async("http://localhost").await.expect_status_no_content();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_no_content();
    ///     surf::get("http://localhost").await.expect_status_no_content();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_no_content();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_no_content();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_no_content();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_no_content();
    ///     });
    /// }
    /// ```
    fn expect_status_no_content(&mut self) -> &mut T { self.expect_status_eq(204) }

    /// Expects response status to be `Partial Content 206`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_partial_content();
    ///     reqwest::blocking::get("http://localhost").expect_status_partial_content();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_partial_content();
    ///     reqwest::get("http://localhost").await.expect_status_partial_content();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_partial_content();
    ///     isahc::get("http://localhost").expect_status_partial_content();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_partial_content();
    ///     isahc::get_async("http://localhost").await.expect_status_partial_content();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_partial_content();
    ///     surf::get("http://localhost").await.expect_status_partial_content();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_partial_content();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_partial_content();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_partial_content();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_partial_content();
    ///     });
    /// }
    /// ```
    fn expect_status_partial_content(&mut self) -> &mut T { self.expect_status_eq(206) }

    /// Expects response status to be `Bad Request 400`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_bad_request();
    ///     reqwest::blocking::get("http://localhost").expect_status_bad_request();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_bad_request();
    ///     reqwest::get("http://localhost").await.expect_status_bad_request();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_bad_request();
    ///     isahc::get("http://localhost").expect_status_bad_request();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_bad_request();
    ///     isahc::get_async("http://localhost").await.expect_status_bad_request();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_bad_request();
    ///     surf::get("http://localhost").await.expect_status_bad_request();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_bad_request();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_bad_request();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_bad_request();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_bad_request();
    ///     });
    /// }
    /// ```
    fn expect_status_bad_request(&mut self) -> &mut T { self.expect_status_eq(400) }

    /// Expects response status to be `Unauthorized 401`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_unauthorized();
    ///     reqwest::blocking::get("http://localhost").expect_status_unauthorized();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_unauthorized();
    ///     reqwest::get("http://localhost").await.expect_status_unauthorized();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_unauthorized();
    ///     isahc::get("http://localhost").expect_status_unauthorized();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_unauthorized();
    ///     isahc::get_async("http://localhost").await.expect_status_unauthorized();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_unauthorized();
    ///     surf::get("http://localhost").await.expect_status_unauthorized();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_unauthorized();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_unauthorized();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_unauthorized();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_unauthorized();
    ///     });
    /// }
    /// ```
    fn expect_status_unauthorized(&mut self) -> &mut T { self.expect_status_eq(401) }

    /// Expects response status to be `Forbidden 403`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_forbidden();
    ///     reqwest::blocking::get("http://localhost").expect_status_forbidden();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_forbidden();
    ///     reqwest::get("http://localhost").await.expect_status_forbidden();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_forbidden();
    ///     isahc::get("http://localhost").expect_status_forbidden();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_forbidden();
    ///     isahc::get_async("http://localhost").await.expect_status_forbidden();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_forbidden();
    ///     surf::get("http://localhost").await.expect_status_forbidden();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_forbidden();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_forbidden();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_forbidden();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_forbidden();
    ///     });
    /// }
    /// ```
    fn expect_status_forbidden(&mut self) -> &mut T { self.expect_status_eq(403) }

    /// Expects response status to be `Not Found 404`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_not_found();
    ///     reqwest::blocking::get("http://localhost").expect_status_not_found();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_not_found();
    ///     reqwest::get("http://localhost").await.expect_status_not_found();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_not_found();
    ///     isahc::get("http://localhost").expect_status_not_found();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_not_found();
    ///     isahc::get_async("http://localhost").await.expect_status_not_found();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_not_found();
    ///     surf::get("http://localhost").await.expect_status_not_found();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_not_found();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_not_found();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_not_found();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_not_found();
    ///     });
    /// }
    /// ```
    fn expect_status_not_found(&mut self) -> &mut T { self.expect_status_eq(404) }

    /// Expects response status to be `Conflict 409`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_conflict();
    ///     reqwest::blocking::get("http://localhost").expect_status_conflict();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_conflict();
    ///     reqwest::get("http://localhost").await.expect_status_conflict();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_conflict();
    ///     isahc::get("http://localhost").expect_status_conflict();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_conflict();
    ///     isahc::get_async("http://localhost").await.expect_status_conflict();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_conflict();
    ///     surf::get("http://localhost").await.expect_status_conflict();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_conflict();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_conflict();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_conflict();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_conflict();
    ///     });
    /// }
    /// ```
    fn expect_status_conflict(&mut self) -> &mut T { self.expect_status_eq(409) }

    /// Expects response status to be `Gone 410`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_gone();
    ///     reqwest::blocking::get("http://localhost").expect_status_gone();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_gone();
    ///     reqwest::get("http://localhost").await.expect_status_gone();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_gone();
    ///     isahc::get("http://localhost").expect_status_gone();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_gone();
    ///     isahc::get_async("http://localhost").await.expect_status_gone();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_gone();
    ///     surf::get("http://localhost").await.expect_status_gone();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_gone();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_gone();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_gone();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_gone();
    ///     });
    /// }
    /// ```
    fn expect_status_gone(&mut self) -> &mut T { self.expect_status_eq(410) }

    /// Expects response status to be `Internal Server Error 500`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_status_internal_server_error();
    ///     reqwest::blocking::get("http://localhost").expect_status_internal_server_error();
    ///     reqwest::get("http://localhost").await.unwrap().expect_status_internal_server_error();
    ///     reqwest::get("http://localhost").await.expect_status_internal_server_error();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_status_internal_server_error();
    ///     isahc::get("http://localhost").expect_status_internal_server_error();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_status_internal_server_error();
    ///     isahc::get_async("http://localhost").await.expect_status_internal_server_error();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_status_internal_server_error();
    ///     surf::get("http://localhost").await.expect_status_internal_server_error();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_status_internal_server_error();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_internal_server_error();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_status_internal_server_error();
    ///         awc::Client::default().get("http://localhost").send().await.expect_status_internal_server_error();
    ///     });
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
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_header("content-type", "application/json");
    ///     reqwest::blocking::get("http://localhost").expect_header("content-type", "application/json");
    ///     reqwest::get("http://localhost").await.unwrap().expect_header("content-type", "application/json");
    ///     reqwest::get("http://localhost").await.expect_header("content-type", "application/json");
    ///
    ///     isahc::get("http://localhost").unwrap().expect_header("content-type", "application/json");
    ///     isahc::get("http://localhost").expect_header("content-type", "application/json");
    ///     isahc::get_async("http://localhost").await.unwrap().expect_header("content-type", "application/json");
    ///     isahc::get_async("http://localhost").await.expect_header("content-type", "application/json");
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_header("content-type", "application/json");
    ///     surf::get("http://localhost").await.expect_header("content-type", "application/json");
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_header("content-type", "application/json");
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header("content-type", "application/json");
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_header("content-type", "application/json");
    ///         awc::Client::default().get("http://localhost").send().await.expect_header("content-type", "application/json");
    ///     });
    /// }
    /// ```
    fn expect_header<'a, K: Into<&'a str>, V: Into<&'a str>>(&mut self, key: K, value: V) -> &mut T;

    /// Expects response multi valued headers to be equal
    /// * `key` - expected header key
    /// * `value` - expected header values
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     reqwest::blocking::get("http://localhost").expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     reqwest::get("http://localhost").await.unwrap().expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     reqwest::get("http://localhost").await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///
    ///     isahc::get("http://localhost").unwrap().expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     isahc::get("http://localhost").expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     isahc::get_async("http://localhost").await.unwrap().expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     isahc::get_async("http://localhost").await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     surf::get("http://localhost").await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_headers("cache-control", ["no-cache", "no-store"]);
    ///         awc::Client::default().get("http://localhost").send().await.expect_headers("cache-control", ["no-cache", "no-store"]);
    ///     });
    /// }
    /// ```
    fn expect_headers<'a, K: Into<&'a str>, V: Into<Vec<&'a str>>>(&mut self, key: K, value: V) -> &mut T;

    /// Expects response header to be present
    /// * `key` - expected present header key
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_header_present("content-type");
    ///     reqwest::blocking::get("http://localhost").expect_header_present("content-type");
    ///     reqwest::get("http://localhost").await.unwrap().expect_header_present("content-type");
    ///     reqwest::get("http://localhost").await.expect_header_present("content-type");
    ///
    ///     isahc::get("http://localhost").unwrap().expect_header_present("content-type");
    ///     isahc::get("http://localhost").expect_header_present("content-type");
    ///     isahc::get_async("http://localhost").await.unwrap().expect_header_present("content-type");
    ///     isahc::get_async("http://localhost").await.expect_header_present("content-type");
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_header_present("content-type");
    ///     surf::get("http://localhost").await.expect_header_present("content-type");
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_header_present("content-type");
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header_present("content-type");
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_header_present("content-type");
    ///         awc::Client::default().get("http://localhost").send().await.expect_header_present("content-type");
    ///     });
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
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_header_absent("content-type");
    ///     reqwest::blocking::get("http://localhost").expect_header_absent("content-type");
    ///     reqwest::get("http://localhost").await.unwrap().expect_header_absent("content-type");
    ///     reqwest::get("http://localhost").await.expect_header_absent("content-type");
    ///
    ///     isahc::get("http://localhost").unwrap().expect_header_absent("content-type");
    ///     isahc::get("http://localhost").expect_header_absent("content-type");
    ///     isahc::get_async("http://localhost").await.unwrap().expect_header_absent("content-type");
    ///     isahc::get_async("http://localhost").await.expect_header_absent("content-type");
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_header_absent("content-type");
    ///     surf::get("http://localhost").await.expect_header_absent("content-type");
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_header_absent("content-type");
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_header_absent("content-type");
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_header_absent("content-type");
    ///         awc::Client::default().get("http://localhost").send().await.expect_header_absent("content-type");
    ///     });
    /// }
    /// ```
    fn expect_header_absent<'a, K: Into<&'a str>>(&mut self, key: K) -> &mut T;

    /// Expects response header `Content-Type: application/json`
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_content_type_json();
    ///     reqwest::blocking::get("http://localhost").expect_content_type_json();
    ///     reqwest::get("http://localhost").await.unwrap().expect_content_type_json();
    ///     reqwest::get("http://localhost").await.expect_content_type_json();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_content_type_json();
    ///     isahc::get("http://localhost").expect_content_type_json();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_content_type_json();
    ///     isahc::get_async("http://localhost").await.expect_content_type_json();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_content_type_json();
    ///     surf::get("http://localhost").await.expect_content_type_json();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_content_type_json();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_content_type_json();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_content_type_json();
    ///         awc::Client::default().get("http://localhost").send().await.expect_content_type_json();
    ///     });
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
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_content_type_text();
    ///     reqwest::blocking::get("http://localhost").expect_content_type_text();
    ///     reqwest::get("http://localhost").await.unwrap().expect_content_type_text();
    ///     reqwest::get("http://localhost").await.expect_content_type_text();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_content_type_text();
    ///     isahc::get("http://localhost").expect_content_type_text();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_content_type_text();
    ///     isahc::get_async("http://localhost").await.expect_content_type_text();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_content_type_text();
    ///     surf::get("http://localhost").await.expect_content_type_text();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_content_type_text();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_content_type_text();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_content_type_text();
    ///         awc::Client::default().get("http://localhost").send().await.expect_content_type_text();
    ///     });
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
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// # use serde::Deserialize;
    /// use asserhttp::*;
    /// use serde_json::{json, Value};
    ///
    /// #[derive(Deserialize, Debug, Eq, PartialEq)]
    /// struct MyStruct { a: String }
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     reqwest::blocking::get("http://localhost").expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     reqwest::get("http://localhost").await.unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     reqwest::get("http://localhost").await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///
    ///     isahc::get("http://localhost").unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     isahc::get("http://localhost").expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     isahc::get_async("http://localhost").await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     surf::get("http://localhost").await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///         awc::Client::default().get("http://localhost").send().await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    ///     });
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
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    /// use serde_json::json;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_body_json_eq(json!({"a": "b"}));
    ///     reqwest::blocking::get("http://localhost").expect_body_json_eq(json!({"a": "b"}));
    ///     reqwest::get("http://localhost").await.unwrap().expect_body_json_eq(json!({"a": "b"}));
    ///     reqwest::get("http://localhost").await.expect_body_json_eq(json!({"a": "b"}));
    ///
    ///     isahc::get("http://localhost").unwrap().expect_body_json_eq(json!({"a": "b"}));
    ///     isahc::get("http://localhost").expect_body_json_eq(json!({"a": "b"}));
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_json_eq(json!({"a": "b"}));
    ///     isahc::get_async("http://localhost").await.expect_body_json_eq(json!({"a": "b"}));
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_json_eq(json!({"a": "b"}));
    ///     surf::get("http://localhost").await.expect_body_json_eq(json!({"a": "b"}));
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_body_json_eq(json!({"a": "b"}));
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_json_eq(json!({"a": "b"}));
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_body_json_eq(json!({"a": "b"}));
    ///         awc::Client::default().get("http://localhost").send().await.expect_body_json_eq(json!({"a": "b"}));
    ///     });
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
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     reqwest::blocking::get("http://localhost").expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     reqwest::get("http://localhost").await.unwrap().expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     reqwest::get("http://localhost").await.expect_body_text(|b| assert_eq!(b, "abcd"));
    ///
    ///     isahc::get("http://localhost").unwrap().expect_body_text(|b| assert_eq!(b, String::from("abcd")));
    ///     isahc::get("http://localhost").unwrap().expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     isahc::get("http://localhost").expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     isahc::get_async("http://localhost").await.expect_body_text(|b| assert_eq!(b, "abcd"));
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     surf::get("http://localhost").await.expect_body_text(|b| assert_eq!(b, "abcd"));
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_text(|b| assert_eq!(b, "abcd"));
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_body_text(|b| assert_eq!(b, "abcd"));
    ///         awc::Client::default().get("http://localhost").send().await.expect_body_text(|b| assert_eq!(b, "abcd"));
    ///     });
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
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_body_text_eq("abcd");
    ///     reqwest::blocking::get("http://localhost").expect_body_text_eq("abcd");
    ///     reqwest::get("http://localhost").await.unwrap().expect_body_text_eq("abcd");
    ///     reqwest::get("http://localhost").await.expect_body_text_eq("abcd");
    ///
    ///     isahc::get("http://localhost").unwrap().expect_body_text_eq(String::from("abcd"));
    ///     isahc::get("http://localhost").unwrap().expect_body_text_eq("abcd");
    ///     isahc::get("http://localhost").expect_body_text_eq("abcd");
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_text_eq("abcd");
    ///     isahc::get_async("http://localhost").await.expect_body_text_eq("abcd");
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_text_eq("abcd");
    ///     surf::get("http://localhost").await.expect_body_text_eq("abcd");
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_body_text_eq("abcd");
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_text_eq("abcd");
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_body_text_eq("abcd");
    ///         awc::Client::default().get("http://localhost").send().await.expect_body_text_eq("abcd");
    ///     });
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
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_body_text_matches("[a-z]+");
    ///     reqwest::blocking::get("http://localhost").expect_body_text_matches("[a-z]+");
    ///     reqwest::get("http://localhost").await.unwrap().expect_body_text_matches("[a-z]+");
    ///     reqwest::get("http://localhost").await.expect_body_text_matches("[a-z]+");
    ///
    ///     isahc::get("http://localhost").unwrap().expect_body_text_matches(String::from("[a-z]+"));
    ///     isahc::get("http://localhost").unwrap().expect_body_text_matches("[a-z]+");
    ///     isahc::get("http://localhost").expect_body_text_matches("[a-z]+");
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_text_matches("[a-z]+");
    ///     isahc::get_async("http://localhost").await.expect_body_text_matches("[a-z]+");
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_text_matches("[a-z]+");
    ///     surf::get("http://localhost").await.expect_body_text_matches("[a-z]+");
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_body_text_matches("[a-z]+");
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_text_matches("[a-z]+");
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_body_text_matches("[a-z]+");
    ///         awc::Client::default().get("http://localhost").send().await.expect_body_text_matches("[a-z]+");
    ///     });
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
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     reqwest::blocking::get("http://localhost").expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     reqwest::get("http://localhost").await.unwrap().expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     reqwest::get("http://localhost").await.expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///
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
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///         awc::Client::default().get("http://localhost").send().await.expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    ///     });
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
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_body_bytes_eq(b"abcd");
    ///     reqwest::blocking::get("http://localhost").expect_body_bytes_eq(b"abcd");
    ///     reqwest::get("http://localhost").await.unwrap().expect_body_bytes_eq(b"abcd");
    ///     reqwest::get("http://localhost").await.expect_body_bytes_eq(b"abcd");
    ///
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
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_body_bytes_eq(b"abcd");
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_bytes_eq(b"abcd");
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_body_bytes_eq(b"abcd");
    ///         awc::Client::default().get("http://localhost").send().await.expect_body_bytes_eq(b"abcd");
    ///     });
    /// }
    /// ```
    fn expect_body_bytes_eq(&mut self, body: &[u8]) -> &mut T {
        self.expect_body_bytes(|actual| assert_bytes_body(actual, body))
    }

    /// Expects a response body to be present and not empty
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_body_present();
    ///     reqwest::blocking::get("http://localhost").expect_body_present();
    ///     reqwest::get("http://localhost").await.unwrap().expect_body_present();
    ///     reqwest::get("http://localhost").await.expect_body_present();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_body_present();
    ///     isahc::get("http://localhost").expect_body_present();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_present();
    ///     isahc::get_async("http://localhost").await.expect_body_present();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_present();
    ///     surf::get("http://localhost").await.expect_body_present();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_body_present();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_present();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_body_present();
    ///         awc::Client::default().get("http://localhost").send().await.expect_body_present();
    ///     });
    /// }
    /// ```
    fn expect_body_present(&mut self) -> &mut T;

    /// Expects a response body to be absent or empty
    ///
    /// # Example
    /// ```no_run
    /// # use isahc;
    /// # use surf;
    /// # use reqwest;
    /// # use hyper;
    /// # use actix_rt::System;
    /// # use awc;
    /// use asserhttp::*;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").unwrap().expect_body_absent();
    ///     reqwest::blocking::get("http://localhost").expect_body_absent();
    ///     reqwest::get("http://localhost").await.unwrap().expect_body_absent();
    ///     reqwest::get("http://localhost").await.expect_body_absent();
    ///
    ///     isahc::get("http://localhost").unwrap().expect_body_absent();
    ///     isahc::get("http://localhost").expect_body_absent();
    ///     isahc::get_async("http://localhost").await.unwrap().expect_body_absent();
    ///     isahc::get_async("http://localhost").await.expect_body_absent();
    ///
    ///     surf::get("http://localhost").await.unwrap().expect_body_absent();
    ///     surf::get("http://localhost").await.expect_body_absent();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.unwrap().expect_body_absent();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_body_absent();
    ///
    ///     System::new("test").block_on(async move {
    ///         awc::Client::default().get("http://localhost").send().await.unwrap().expect_body_absent();
    ///         awc::Client::default().get("http://localhost").send().await.expect_body_absent();
    ///     });
    /// }
    /// ```
    fn expect_body_absent(&mut self) -> &mut T;
}