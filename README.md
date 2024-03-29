<h1 align="center">asserhttp</h1>
<div align="center">
 <strong>
   Fluent http response assertions
 </strong>
</div>
<br />
<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/asserhttp">
    <img src="https://img.shields.io/crates/v/asserhttp.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Minimum Supported Rust Version -->
  <a href="rust: 1.60.0+">
    <img src="https://img.shields.io/badge/rust-1.60.0%2B-green.svg"
    alt="Crates.io version" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/asserhttp">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- license -->
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-Apache_2-blue.svg?style=flat-square"
      alt="Apache 2" />
  </a>
  <!-- CI status -->
  <a href="https://github.com/beltram/asserhttp/actions">
    <img src="https://github.com/beltram/asserhttp/workflows/ci/badge.svg?style=flat-square"
      alt="ci" />
  </a>
  <!-- Code coverage -->
  <a href="https://coveralls.io/github/beltram/asserhttp?branch=main">
    <img src="https://coveralls.io/repos/github/beltram/asserhttp/badge.svg?branch=main" alt="coverage" />
  </a>
</div>
<br/>

A standard trait for doing fluent assertions over many http client response. Currently, supports
[actix-web](https://actix.rs/docs/testing/), [rocket](https://github.com/SergioBenitez/Rocket),
[reqwest](https://github.com/seanmonstar/reqwest), [hyper](https://github.com/hyperium/hyper),
[axum](https://github.com/tokio-rs/axum), [awc](https://docs.rs/awc) (Actix Web Client),
[surf](https://github.com/http-rs/surf), [ureq](https://github.com/algesten/ureq)
and [isahc](https://github.com/sagebind/isahc).

## Getting started

Add it to your `Cargo.toml`

```toml
asserhttp = { version = "0.6.1", features = ["reqwest"] }
#                             or features = ["hyper"]
#                             or features = ["actix"]
#                             or features = ["axum"]
#                             or features = ["actix-web-client"]
#                             or features = ["rocket"]
#                             or features = ["surf"]
#                             or features = ["ureq"]
#                             or features = ["isahc"]
```

Then use it in your tests, for example on [actix-web](https://actix.rs/docs/testing/),

```rust
use actix_web::{App, HttpResponse, test::{call_service, init_service, TestRequest}, web};
use asserhttp::*;

#[actix_web::test]
async fn sample_test() {
    let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body(json!({"a": "b"})) }));
    call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
        .expect_status_ok()
        .expect_content_type_json()
        .expect_body_json_eq(json!({"a": "b"}));
}
```

or on [reqwest](https://github.com/seanmonstar/reqwest)

```rust
use reqwest;
use asserhttp::*;

#[tokio::test]
async fn my_test() {
    reqwest::get("http://localhost").await
        .expect_status_ok()
        .expect_content_type_json()
        .expect_body_json_eq(json!({"name": "jdoe"}));
}
```

## Customize

You don't like the asserhttp methods name ? That's fine, you can define yours. Define you own trait and use asserhttp 
methods to define your own ! 

As simple as this:

```rust
asserhttp::asserhttp_customize!(MyHttpDsl);

pub trait MyHttpDsl<T>: asserhttp::Asserhttp<T> {
    fn is_status_ok(&mut self) -> &mut T {
        self.expect_status_ok()
    }
    fn is_json(&mut self) -> &mut T {
        self.expect_content_type_json()
    }
    fn has_body(&mut self) -> &mut T { self.expect_body_present() }
}
```

## gRPC

Asserting gRPC is also supported with a [tonic](https://github.com/hyperium/tonic) client. Simply turn on the `tonic` 
feature and use it like this:

```rust
use asserhttp::grpc::*;

#[tokio::main]
async fn main() {
    // success
    client.call_svc(tonic::Request::new(Payload)).await.expect_status_ok().expect_body(Payload);
    // error
    client.call_svc(tonic::Request::new(Payload)).await.expect_status_error(Code::NotFound);
}
```
