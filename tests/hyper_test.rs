#[macro_export]
macro_rules! hyper_test {
    ($fn_name:ident, $stub:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<hyper_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                let addr = stubr.uri().parse().unwrap();
                $(crate::hyper_test::hyper_request(addr).await$( .$meth($($arg),*) )+;)+
            }

            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<hyper_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                let addr = stubr.uri().parse().unwrap();
                $(crate::hyper_test::hyper_request(addr).await.unwrap()$( .$meth($($arg),*) )+;)+
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
                let addr = stubr.uri().parse().unwrap();
                $(crate::hyper_test::hyper_request(addr).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<hyper_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                let addr = stubr.uri().parse().unwrap();
                $(crate::hyper_test::hyper_request(addr).await$( .$meth($($arg),*) )+;)+
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
                let addr = stubr.uri().parse().unwrap();
                $(assert_eq!(crate::hyper_test::hyper_request(addr).await.unwrap()$( .$meth($($arg),*) )+.unwrap_err(), $error);)+
            }
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<hyper_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                let addr = stubr.uri().parse().unwrap();
                $(assert_eq!(crate::hyper_test::hyper_request(addr).await$( .$meth($($arg),*) )+.unwrap_err(), $error);)+
            }
        }
    };
}

pub async fn hyper_request<>(uri: String) -> hyper::Result<hyper::Response<impl http_body::Body>> {
    let url = url::Url::parse(&uri).unwrap();

    let host = url.host().expect("uri has no host");
    let port = url.port().unwrap();

    let addr = format!("{host}:{port}");

    let stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    let io = hyper_util::rt::TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Hyper connection failed: {:?}", err);
        }
    });

    let req = hyper::Request::builder()
        .uri(url.as_str())
        .body(http_body_util::Empty::<hyper::body::Bytes>::new())
        .unwrap();

    sender.send_request(req).await
}
