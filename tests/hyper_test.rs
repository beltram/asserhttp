#[macro_export]
macro_rules! hyper_test {
    ($fn_name:ident, $stub:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<hyper_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await$( .$meth($($arg),*) )+;)+
            }
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<hyper_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $stub:literal, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<hyper_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<hyper_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $stub:literal, $error:expr, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<hyper_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(assert_eq!(hyper::Client::new().get(stubr.uri().parse().unwrap()).await.unwrap()$( .$meth($($arg),*) )+.unwrap_err(), $error);)+
            }
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<hyper_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(assert_eq!(hyper::Client::new().get(stubr.uri().parse().unwrap()).await$( .$meth($($arg),*) )+.unwrap_err(), $error);)+
            }
        }
    };
}
