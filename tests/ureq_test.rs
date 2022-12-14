#[macro_export]
macro_rules! ureq_test {
    ($fn_name:ident, $stub:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[test]
            #[stubr::mock($stub)]
            fn [<ureq_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                use ureq::OrAnyStatus;
                $(ureq::get(&stubr.uri()).call().or_any_status()$( .$meth($($arg),*) )+;)+
            }
            #[test]
            #[stubr::mock($stub)]
            fn [<ureq_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                use ureq::OrAnyStatus;
                $(ureq::get(&stubr.uri()).call().or_any_status().unwrap()$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $stub:literal, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<ureq_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                use ureq::OrAnyStatus;
                $(ureq::get(&stubr.uri()).call().or_any_status()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<ureq_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                use ureq::OrAnyStatus;
                $(ureq::get(&stubr.uri()).call().or_any_status().unwrap()$( .$meth($($arg),*) )+;)+
            }
        }
    };
}