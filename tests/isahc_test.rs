#[macro_export]
macro_rules! isahc_test {
    ($fn_name:ident, $stub:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // blocking
            #[test]
            #[stubr::mock($stub)]
            fn [<isahc_blocking_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(isahc::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[test]
            #[stubr::mock($stub)]
            fn [<isahc_blocking_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(isahc::get(stubr.uri())$( .$meth($($arg),*) )+;)+
            }

            // async
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<isahc_async_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(isahc::get_async(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<isahc_async_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(isahc::get_async(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $stub:literal, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // blocking
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<isahc_blocking_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(isahc::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<isahc_blocking_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(isahc::get(stubr.uri())$( .$meth($($arg),*) )+;)+
            }

            // async
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<isahc_async_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(isahc::get_async(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<isahc_async_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(isahc::get_async(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $stub:literal, $error:expr, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // blocking
            #[stubr::mock($stub)]
            #[test]
            fn [<isahc_blocking_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(assert_eq!(isahc::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+.unwrap_err(), $error);)+
            }
            #[stubr::mock($stub)]
            #[test]
            fn [<isahc_blocking_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(assert_eq!(isahc::get(stubr.uri())$( .$meth($($arg),*) )+.unwrap_err(), $error);)+
            }

            // async
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<isahc_async_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(assert_eq!(isahc::get_async(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+.unwrap_err(), $error);)+
            }
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<isahc_async_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(assert_eq!(isahc::get_async(stubr.uri()).await$( .$meth($($arg),*) )+.unwrap_err(), $error);)+
            }
        }
    };
}
