// For axum
#[macro_export]
macro_rules! axum_test {
    ($fn_name:ident, $init:expr, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[tokio::test]
            async fn [<axum_ $fn_name>]() {
                use asserhttp::*;
                use tower::ServiceExt as _;
                let app = axum::Router::new().route("/", axum::routing::get( || async move { $init }));
                let req = axum::http::Request::builder().method(axum::http::Method::GET).uri("/").body(axum::body::Body::empty()).unwrap();
                $(app.oneshot(req).await$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $init:expr, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[should_panic(expected = $panic_msg)]
            #[tokio::test]
            async fn [<axum_ $fn_name>]() {
                use asserhttp::*;
                use tower::ServiceExt as _;
                let app = axum::Router::new().route("/", axum::routing::get( || async move { $init }));
                let req = axum::http::Request::builder().method(axum::http::Method::GET).uri("/").body(axum::body::Body::empty()).unwrap();
                $(app.oneshot(req).await$( .$meth($($arg),*) )+;)+
            }
        }
    };
}