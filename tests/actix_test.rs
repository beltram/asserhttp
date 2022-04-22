// For actix
#[macro_export]
macro_rules! actix_test {
    ($fn_name:ident, $init:expr, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // unit
            #[actix_web::test]
            async fn [<actix_unit_ $fn_name>]() {
                use asserhttp::*;
                use actix_web::{HttpRequest, HttpResponse, test::TestRequest};
                async fn endpoint(_: HttpRequest) -> HttpResponse { $init }
                $(endpoint(TestRequest::get().to_http_request()).await$( .$meth($($arg),*) )+;)+
            }

            // integration
            #[actix_web::test]
            async fn [<actix_integration_ $fn_name>]() {
                use asserhttp::*;
                use actix_web::{App, test::{call_service, init_service, TestRequest}, web};
                let app = App::new().route("/", web::get().to(|| async { $init }));
                $(call_service(&init_service(app).await, TestRequest::get().to_request()).await$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $init:expr, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // unit
            #[should_panic(expected = $panic_msg)]
            #[actix_web::test]
            async fn [<actix_unit_ $fn_name>]() {
                use asserhttp::*;
                use actix_web::{HttpRequest, HttpResponse, test::TestRequest};
                async fn endpoint(_: HttpRequest) -> HttpResponse { $init }
                $(endpoint(TestRequest::get().to_http_request()).await$( .$meth($($arg),*) )+;)+
            }

            // integration
            #[should_panic(expected = $panic_msg)]
            #[actix_web::test]
            async fn [<actix_integration_ $fn_name>]() {
                use asserhttp::*;
                use actix_web::{App, test::{call_service, init_service, TestRequest}, web};
                let app = App::new().route("/", web::get().to(|| async { $init }));
                $(call_service(&init_service(app).await, TestRequest::get().to_request()).await$( .$meth($($arg),*) )+;)+
            }
        }
    };
}