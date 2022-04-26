#[macro_export]
macro_rules! surf_test {
    ($fn_name:ident, $stub:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
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
        }
    };
    ($fn_name:ident, $stub:literal, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
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
        }
    };
}