use std::io::Cursor;

use rocket::{http::Header, Request, response, Response, response::Responder};
use serde_json::Value;

pub struct Resp(pub Response<'static>);

impl Responder<'_, 'static> for Resp {
    fn respond_to(self, _: &'_ Request<'_>) -> response::Result<'static> {
        Ok(self.0)
    }
}

impl From<rocket::http::Status> for Resp {
    fn from(status: rocket::http::Status) -> Self {
        Self(Response::build().status(status).finalize())
    }
}

impl From<Vec<(&'static str, Vec<&'static str>)>> for Resp {
    fn from(headers: Vec<(&'static str, Vec<&'static str>)>) -> Self {
        let mut builder = Response::build();
        for (key, values) in headers {
            for v in values {
                builder.header_adjoin(Header::new(key, v));
            }
        }
        Self(builder.finalize())
    }
}

impl From<Vec<(&'static str, &'static str)>> for Resp {
    fn from(headers: Vec<(&'static str, &'static str)>) -> Self {
        Self::from(headers.into_iter().map(|(k, v)| (k, vec![v])).collect::<Vec<_>>())
    }
}

impl From<Value> for Resp {
    fn from(json: Value) -> Self {
        Self(Response::build().streamed_body(Cursor::new(json.to_string())).finalize())
    }
}

impl From<&'static str> for Resp {
    fn from(body: &'static str) -> Self {
        Self(Response::build().streamed_body(Cursor::new(body)).finalize())
    }
}

// For rocket
#[macro_export]
macro_rules! rocket_test {
    ($fn_name:ident, $init:expr, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // blocking
            #[test]
            fn [<rocket_blocking_ $fn_name>]() {
                use asserhttp::*;
                #[rocket::get("/")]
                fn endpoint() -> $crate::Resp { $crate::Resp::from($init) }
                let client = rocket::local::blocking::Client::tracked(rocket::build().mount("/", rocket::routes![endpoint])).unwrap();
                $(client.get("/").dispatch()$( .$meth($($arg),*) )+;)+
            }

            // async
            #[rocket::async_test]
            async fn [<rocket_async_ $fn_name>]() {
                use asserhttp::*;
                #[rocket::get("/")]
                fn endpoint() -> $crate::Resp { $crate::Resp::from($init) }
                let client = rocket::local::asynchronous::Client::tracked(rocket::build().mount("/", rocket::routes![endpoint])).await.unwrap();
                $(client.get("/").dispatch().await$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $init:expr, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // blocking
            #[should_panic(expected = $panic_msg)]
            #[test]
            fn [<rocket_blocking_ $fn_name>]() {
                use asserhttp::*;
                #[rocket::get("/")]
                fn endpoint() -> $crate::Resp { $crate::Resp::from($init) }
                let client = rocket::local::blocking::Client::tracked(rocket::build().mount("/", rocket::routes![endpoint])).unwrap();
                $(client.get("/").dispatch()$( .$meth($($arg),*) )+;)+
            }

            // async
            #[should_panic(expected = $panic_msg)]
            #[rocket::async_test]
            async fn [<rocket_async_ $fn_name>]() {
                use asserhttp::*;
                #[rocket::get("/")]
                fn endpoint() -> $crate::Resp { $crate::Resp::from($init) }
                let client = rocket::local::asynchronous::Client::tracked(rocket::build().mount("/", rocket::routes![endpoint])).await.unwrap();
                $(client.get("/").dispatch().await$( .$meth($($arg),*) )+;)+
            }
        }
    };
}