use actix_web::{App, HttpResponse, test::{call_service, init_service, TestRequest}, web};
use serde_json::{json, Value};

use asserhttp::*;

use super::super::super::common::Body;

mod json {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_raw_body_json() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body(json!({"a": "b"})) }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    }

    #[actix_rt::test]
    async fn should_expect_struct_body_json() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body(json!({"a": "b"})) }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_json(|b: Body| assert_eq!(b, Body { a: String::from("b") }));
    }

    #[should_panic]
    #[actix_rt::test]
    async fn expect_body_json_should_fail_when_closure_fails() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body(json!({"a": "b"})) }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_json(|b: Value| assert_eq!(b, json!({"a": "c"})));
    }
}

mod json_eq {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_raw_body_json() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body(json!({"a": "b"})) }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_json_eq(json!({"a": "b"}));
    }

    #[actix_rt::test]
    async fn should_expect_struct_body_json() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body(json!({"a": "b"})) }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_json_eq(Body { a: String::from("b") });
    }

    #[should_panic(expected = "expected json body Object({\"a\": String(\"b\")}) to be equal to Object({\"a\": String(\"c\")}) but was not")]
    #[actix_rt::test]
    async fn expect_body_json_should_fail_when_not_equal() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body(json!({"a": "b"})) }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_json_eq(json!({"a": "c"}));
    }

    #[should_panic(expected = "expected a json body but no response body was present")]
    #[actix_rt::test]
    async fn expect_body_json_should_fail_when_missing() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_json_eq(json!({"a": "b"}));
    }
}

mod text {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_body_text() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("abcd") }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_text(|b| assert_eq!(b, String::from("abcd")));
    }

    #[should_panic]
    #[actix_rt::test]
    async fn expect_body_text_should_fail_when_closure_fails() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("abcd") }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_text(|b| assert_eq!(b, String::from("dcba")));
    }
}

mod text_eq {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_body_text_eq() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("abcd") }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_text_eq("abcd");
    }

    #[should_panic(expected = "expected text body 'abcd' to be equal to 'dcab' but was not")]
    #[actix_rt::test]
    async fn expect_body_text_should_fail_when_not_equal() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("abcd") }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_text_eq("dcab");
    }

    #[should_panic(expected = "expected a text body but no response body was present")]
    #[actix_rt::test]
    async fn expect_body_text_should_fail_when_missing() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_text_eq("abcd");
    }
}

mod regex {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_body_text_matches() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("abcd") }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_text_matches("[a-d]+");
    }

    #[should_panic(expected = "expected text body 'abcd' to match regex '[e-h]+' but did not")]
    #[actix_rt::test]
    async fn expect_body_text_matches_should_fail_when_does_not_match_regex() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("abcd") }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_text_matches("[e-h]+");
    }

    #[should_panic(expected = "expected a text body but no response body was present")]
    #[actix_rt::test]
    async fn expect_body_text_should_fail_when_missing() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_text_matches("[a-d]+");
    }
}

mod bytes {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_body_bytes() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("abcd") }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    }

    #[should_panic]
    #[actix_rt::test]
    async fn expect_body_bytes_should_fail_when_closure_fails() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("abcd") }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_bytes(|b| assert_eq!(b, b"dcba"));
    }
}

mod bytes_eq {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_body_bytes_eq() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("abcd") }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_bytes_eq(b"abcd");
    }

    #[should_panic(expected = "expected body '[97, 98, 99, 100]' to be equal to '[100, 99, 98, 97]' but was not")]
    #[actix_rt::test]
    async fn expect_body_bytes_should_fail_not_equal() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("abcd") }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_bytes_eq(b"dcba");
    }

    #[should_panic(expected = "expected a response body but no response body was present")]
    #[actix_rt::test]
    async fn expect_body_bytes_should_fail_when_missing() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_body_bytes_eq(b"abcd");
    }
}

mod present {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_body_present() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("abcd") }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_body_present();
    }

    #[should_panic(expected = "expected a response body but no response body was present")]
    #[actix_rt::test]
    async fn expect_body_present_should_fail_when_absent() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_body_present();
    }
}

mod absent {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_body_absent() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_body_absent();
    }

    #[should_panic(expected = "expected no response body but a response body was present")]
    #[actix_rt::test]
    async fn expect_body_absent_should_fail_when_present() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("abcd") }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_body_absent();
    }
}