#[macro_export]
macro_rules! reqwest_test {
    ($fn_name:ident, $stub:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // blocking
            #[test]
            #[stubr::mock($stub)]
            fn [<reqwest_blocking_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(reqwest::blocking::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[test]
            #[stubr::mock($stub)]
            fn [<reqwest_blocking_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(reqwest::blocking::get(stubr.uri())$( .$meth($($arg),*) )+;)+
            }

            // async
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<reqwest_async_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(reqwest::get(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<reqwest_async_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(reqwest::get(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $stub:literal, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // blocking
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<reqwest_blocking_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(reqwest::blocking::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<reqwest_blocking_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(reqwest::blocking::get(stubr.uri())$( .$meth($($arg),*) )+;)+
            }

            // async
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<reqwest_async_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(reqwest::get(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<reqwest_async_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(reqwest::get(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }
        }
    };
}
