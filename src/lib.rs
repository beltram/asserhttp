//!
//! Allows fluent assertions for various http client responses.
//! Supports [actix-web](https://actix.rs/docs/testing/), [rocket](https://github.com/SergioBenitez/Rocket),
//! [reqwest](https://github.com/seanmonstar/reqwest), [hyper](https://github.com/hyperium/hyper),
//! [awc](https://docs.rs/awc) (Actix Web Client), [surf](https://github.com/http-rs/surf) and [isahc](https://github.com/sagebind/isahc).
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
//!     .expect_status(200)
//!     .expect_status(Status::Ok)
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
//!     .expect_header(headers::CONTENT_TYPE, "application/pdf")
//!     .expect_header("content-type", |h| assert_eq!(h, "application/pdf"))
//!     .expect_headers("cache-control", ["no-cache", "no-store"])
//!     .expect_headers(headers::CACHE_CONTROL, ["no-cache", "no-store"])
//!     .expect_headers("cache-control", |h: Vec<&str>| assert!(h.contains(&"a") && h.contains(&"b")))
//!     .expect_header_present("x-my-header")
//!     .expect_header_present(headers::CONTENT_TYPE)
//!     .expect_header_absent("x-my-header")
//!     .expect_header_absent(headers::ACCEPT)
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
//! ## [actix-web](https://actix.rs/docs/testing/)
//!
//! Use `actix` feature.
//!
//! For unit testing
//!
//! ```no_run
//! use actix_web::{HttpRequest, HttpResponse, test::TestRequest};
//! use serde_json::json;
//! use asserhttp::*;
//!
//! #[actix_web::test]
//! async fn sample_test() {
//!     async fn handler(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().body(json!({"a": "b"})) }
//!     handler(TestRequest::get().to_http_request()).await
//!         .expect_status_ok()
//!         .expect_content_type_json()
//!         .expect_body_json_eq(json!({"a": "b"}));
//!     // and many more !
//! }
//! ```
//!
//! For integration tests
//!
//! ```no_run
//! use actix_web::{App, HttpResponse, test::{call_service, init_service, TestRequest}, web};
//! use serde_json::json;
//! use asserhttp::*;
//!
//! #[actix_web::test]
//! async fn sample_test() {
//!     let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body(json!({"a": "b"})) }));
//!     call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
//!         .expect_status_ok()
//!         .expect_content_type_json()
//!         .expect_body_json_eq(json!({"a": "b"}));
//!     // and many more !
//! }
//! ```
//!
//! ## rocket
//!
//! Use `rocket` feature.
//!
//! ```no_run
//! use rocket::{get, http::Status, local::asynchronous::Client, routes};
//! use serde_json::{json, Value};
//! use asserhttp::*;
//!
//! #[rocket::async_test]
//! async fn sample_test() {
//!     #[get("/")]
//!     fn endpoint() -> Value { json!({"a": "b"}) }
//!     let client = Client::tracked(rocket::build().mount("/", routes![endpoint])).await.unwrap();
//!     client.get("/").dispatch().await
//!         .expect_status_ok()
//!         .expect_content_type_json()
//!         .expect_body_json_eq(json!({"a": "b"}));
//! }
//! ```
//!
//! ## reqwest
//!
//! Use `reqwest` feature.
//!
//! ```no_run
//! use reqwest;
//! use asserhttp::*;
//! use serde_json::json;
//!
//! #[tokio::test]
//! async fn sample_test() {
//!     reqwest::get("http://localhost/api/any").await.unwrap().expect_status_ok();
//!     // no need to call `.unwrap()` directly
//!     reqwest::get("http://localhost/api/any").await.expect_status(200);
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
//! Use `hyper` feature.
//!
//! ```no_run
//! use hyper::Client;
//! use asserhttp::*;
//! use serde_json::json;
//!
//! #[tokio::test]
//! async fn sample_test() {
//!     Client::new().get("http://localhost/api/any").await.unwrap().expect_status_ok();
//!     // no need to call `.unwrap()` directly
//!     Client::new().get("http://localhost/api/any").await.expect_status(200);
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
//! ## axum
//!
//! Use `axum` feature.
//!
//! ```no_run
//! use asserhttp::*;
//! use serde_json::json;
//! use tower::ServiceExt as _;
//!
//! #[tokio::test]
//! async fn sample_test() {
//!     let app = axum::Router::new().route("/", axum::routing::get( || async move {
//!         let mut axum_response = axum::response::Response::builder()
//!             .status(axum::http::StatusCode::OK)
//!             .body(json!({"a": "b"}).to_string())
//!             .unwrap();
//!         axum_response.headers_mut().insert("content-type", "application/json".parse().unwrap());
//!         axum_response
//!     }));
//!     let req = axum::http::Request::builder().method(axum::http::Method::GET).uri("/").body(axum::body::Body::empty()).unwrap();
//!     app.oneshot(req).await
//!         .expect_status_ok()
//!         .expect_content_type_json()
//!         .expect_body_json_eq(json!({"name": "jdoe"}));
//! }
//! ```
//!
//! ## awc (Actix Web Client)
//!
//! Use `actix-web-client` feature.
//!
//! ```no_run
//! use awc::Client;
//! use asserhttp::*;
//! use serde_json::json;
//!
//! #[actix_web::test]
//! async fn sample_test() {
//!     Client::default().get("http://localhost/api/any").await.unwrap().expect_status_ok();
//!     // no need to call `.unwrap()` directly
//!     Client::default().get("http://localhost/api/any").await.expect_status(200);
//!     Client::default().get("http://localhost/api/any").await.expect_status_ok();
//!     Client::default().get("http://localhost/api/any").await.expect_status_bad_request();
//!     Client::default().get("http://localhost/api/any").await.expect_status_internal_server_error();
//!     // chain expectations
//!     Client::default().get("http://localhost/api/any").await
//!         .expect_status_ok()
//!         .expect_content_type_json()
//!         .expect_body_json_eq(json!({"name": "jdoe"}));
//!     // and many more !
//! }
//! ```
//!
//! ## surf
//!
//! Use `surf` feature.
//!
//! ```no_run
//! use surf;
//! use asserhttp::*;
//! use serde_json::json;
//!
//! #[tokio::test]
//! async fn sample_test() {
//!     surf::get("http://localhost/api/any").await.unwrap().expect_status_ok();
//!     // no need to call `.unwrap()` directly
//!     surf::get("http://localhost/api/any").await.expect_status(200);
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
//! ## ureq
//!
//! Use `ureq` feature.
//!
//! ```no_run
//! use ureq::OrAnyStatus;
//! use asserhttp::*;
//! use serde_json::json;
//!
//! #[tokio::test]
//! async fn sample_test() {
//!     ureq::get("http://localhost/api/any").call().or_any_status().unwrap().expect_status_ok();
//!     // no need to call `.unwrap()` directly
//!     ureq::get("http://localhost/api/any").call().or_any_status().expect_status(200);
//!     ureq::get("http://localhost/api/any").call().or_any_status().expect_status_ok();
//!     ureq::get("http://localhost/api/any").call().or_any_status().expect_status_bad_request();
//!     ureq::get("http://localhost/api/any").call().or_any_status().expect_status_internal_server_error();
//!     // chain expectations
//!     ureq::get("http://localhost/api/any").call().or_any_status()
//!         .expect_status_ok()
//!         .expect_content_type_json()
//!         .expect_body_json_eq(json!({"name": "jdoe"}));
//!     // and many more !
//! }
//! ```
//!
//! ## isahc
//!
//! Use `isahc` feature.
//!
//! ```no_run
//! use isahc;
//! use asserhttp::*;
//! use serde_json::json;
//!
//! #[tokio::test]
//! async fn sample_test() {
//!     isahc::get_async("http://localhost/api/any").await.unwrap().expect_status_ok();
//!     // no need to call `.unwrap()` directly
//!     isahc::get_async("http://localhost/api/any").await.expect_status(200);
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

pub use body::AsserhttpBody;
pub use header::AsserhttpHeader;
pub use status::AsserhttpStatus;
pub use http_types::StatusCode as Status;
pub use http_types::headers as headers;

#[cfg(feature = "surf")]
mod assert_surf;
#[cfg(feature = "isahc")]
mod assert_isahc;
#[cfg(feature = "reqwest")]
mod assert_reqwest;
#[cfg(feature = "hyper")]
mod assert_hyper;
#[cfg(feature = "axum")]
mod assert_axum;
#[cfg(feature = "actix-web-client")]
mod assert_awc;
#[cfg(feature = "actix")]
mod assert_actix;
#[cfg(feature = "rocket")]
mod assert_rocket;
#[cfg(feature = "ureq")]
mod assert_ureq;

mod accessor;
mod status;
mod header;
mod body;

/// For assertions on http response
pub trait Asserhttp<T>: AsserhttpStatus<T> + AsserhttpHeader<T> + AsserhttpBody<T> {}