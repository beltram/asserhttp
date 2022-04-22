// For reqwest, surf, isahc, hyper & awc
#[macro_export]
macro_rules! http_client_test {
    ($fn_name:ident, $stub:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // reqwest blocking
            #[test]
            #[stubr::mock($stub)]
            fn [<reqwest_blocking_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::blocking::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[test]
            #[stubr::mock($stub)]
            fn [<reqwest_blocking_result_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::blocking::get(stubr.uri())$( .$meth($($arg),*) )+;)+
            }

            // reqwest async
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<reqwest_async_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::get(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<reqwest_async_result_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::get(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }

            // surf
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<surf_ $fn_name>]() {
                use asserhttp::*;
                $(surf::get(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<surf_result_ $fn_name>]() {
                use asserhttp::*;
                $(surf::get(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }

            // hyper
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<hyper_ $fn_name>]() {
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await$( .$meth($($arg),*) )+;)+
            }
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<hyper_result_ $fn_name>]() {
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }

            // awc
            #[actix_web::test]
            #[stubr::mock($stub)]
            async fn [<awc_ $fn_name>]() {
                use asserhttp::*;
                $(awc::Client::default().get(stubr.uri()).send().await$( .$meth($($arg),*) )+;)+
            }
            #[actix_web::test]
            #[stubr::mock($stub)]
            async fn [<awc_result_ $fn_name>]() {
                use asserhttp::*;
                $(awc::Client::default().get(stubr.uri()).send().await.unwrap()$( .$meth($($arg),*) )+;)+
            }

            // isahc blocking
            #[test]
            #[stubr::mock($stub)]
            fn [<isahc_blocking_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[test]
            #[stubr::mock($stub)]
            fn [<isahc_blocking_result_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get(stubr.uri())$( .$meth($($arg),*) )+;)+
            }

            // isahc async
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<isahc_async_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get_async(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<isahc_async_result_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get_async(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $stub:literal, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // reqwest blocking
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<reqwest_blocking_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::blocking::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<reqwest_blocking_result_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::blocking::get(stubr.uri())$( .$meth($($arg),*) )+;)+
            }

            // reqwest async
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<reqwest_async_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::get(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<reqwest_async_result_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::get(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }

            // surf
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<surf_ $fn_name>]() {
                use asserhttp::*;
                $(surf::get(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<surf_result_ $fn_name>]() {
                use asserhttp::*;
                $(surf::get(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }

            // hyper
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<hyper_ $fn_name>]() {
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<hyper_result_ $fn_name>]() {
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await$( .$meth($($arg),*) )+;)+
            }

            // awc
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[actix_web::test]
            async fn [<awc_ $fn_name>]() {
                use asserhttp::*;
                $(awc::Client::default().get(stubr.uri()).send().await$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[actix_web::test]
            async fn [<awc_result_ $fn_name>]() {
                use asserhttp::*;
                $(awc::Client::default().get(stubr.uri()).send().await.unwrap()$( .$meth($($arg),*) )+;)+
            }

            // isahc blocking
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<isahc_blocking_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<isahc_blocking_result_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get(stubr.uri())$( .$meth($($arg),*) )+;)+
            }

            // isahc async
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<isahc_async_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get_async(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<isahc_async_result_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get_async(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }
        }
    };
}