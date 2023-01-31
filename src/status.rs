use std::str::FromStr;

/// For assertions on http response status
pub trait AsserhttpStatus<T> {
    /// DEPRECATED
    /// Expects response status to be equal
    /// * `status` - expected status
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_eq(200);
    ///     reqwest::blocking::get("http://localhost").expect_status_eq(Status::Ok);
    ///     reqwest::get("http://localhost").await.expect_status_eq(200);
    ///     reqwest::get("http://localhost").await.expect_status_eq(Status::Ok);
    ///
    ///     isahc::get("http://localhost").expect_status_eq(200);
    ///     isahc::get("http://localhost").expect_status_eq(Status::Ok);
    ///     isahc::get_async("http://localhost").await.expect_status_eq(200);
    ///     isahc::get_async("http://localhost").await.expect_status_eq(Status::Ok);
    ///
    ///     surf::get("http://localhost").await.expect_status_eq(200);
    ///     surf::get("http://localhost").await.expect_status_eq(Status::Ok);
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_eq(200);
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_eq(Status::Ok);
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_eq(200);
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_eq(Status::Ok);
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_eq(200);
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_eq(Status::Ok);
    /// }
    /// ```
    #[deprecated(since = "0.5.0", note = "Use 'expect_status' instead with same signature")]
    fn expect_status_eq(&mut self, status: impl Into<AnyStatus>) -> &mut T {
        self.expect_status(status)
    }

    /// Expects response status to be equal
    /// * `status` - expected status or asserhttp::Status or closure
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status(200);
    ///     reqwest::blocking::get("http://localhost").expect_status(Status::Ok);
    ///     reqwest::blocking::get("http://localhost").expect_status(|s| assert_eq!(s, 200));
    ///     reqwest::get("http://localhost").await.expect_status(200);
    ///     reqwest::get("http://localhost").await.expect_status(Status::Ok);
    ///     reqwest::get("http://localhost").await.expect_status(|s| assert_eq!(s, 200));
    ///
    ///     isahc::get("http://localhost").expect_status(200);
    ///     isahc::get("http://localhost").expect_status(Status::Ok);
    ///     isahc::get("http://localhost").expect_status(|s| assert_eq!(s, 200));
    ///     isahc::get_async("http://localhost").await.expect_status(200);
    ///     isahc::get_async("http://localhost").await.expect_status(Status::Ok);
    ///     isahc::get_async("http://localhost").await.expect_status(|s| assert_eq!(s, 200));
    ///
    ///     surf::get("http://localhost").await.expect_status(200);
    ///     surf::get("http://localhost").await.expect_status(Status::Ok);
    ///     surf::get("http://localhost").await.expect_status(|s| assert_eq!(s, 200));
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status(200);
    ///     ureq::get("http://localhost").call().or_any_status().expect_status(Status::Ok);
    ///     ureq::get("http://localhost").call().or_any_status().expect_status(|s| assert_eq!(s, 200));
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status(200);
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status(Status::Ok);
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status(|s| assert_eq!(s, 200));
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status(200);
    ///     awc::Client::default().get("http://localhost").send().await.expect_status(Status::Ok);
    ///     awc::Client::default().get("http://localhost").send().await.expect_status(|s| assert_eq!(s, 200));
    /// }
    /// ```
    fn expect_status(&mut self, status: impl Into<AnyStatus>) -> &mut T;

    /// Expects response status to be in range
    /// * `lower` - lower inclusive bound
    /// * `upper` - upper exclusive bound
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_in_range(200, 400);
    ///     reqwest::get("http://localhost").await.expect_status_in_range(200, 400);
    ///
    ///     isahc::get("http://localhost").expect_status_in_range(200, 400);
    ///     isahc::get_async("http://localhost").await.expect_status_in_range(200, 400);
    ///
    ///     surf::get("http://localhost").await.expect_status_in_range(200, 400);
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_in_range(200, 400);
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_in_range(200, 400);
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_in_range(200, 400);
    /// }
    /// ```
    fn expect_status_in_range(&mut self, lower: u16, upper: u16) -> &mut T {
        self.expect_status(move |status: u16| {
            assert!(
                status.ge(&lower) && status.lt(&upper),
                "expected status to be in [{};{}[ but was '{}'",
                lower,
                upper,
                status
            );
        })
    }

    /// Expects response status to be in 2xx range
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_success();
    ///     reqwest::get("http://localhost").await.expect_status_success();
    ///
    ///     isahc::get("http://localhost").expect_status_success();
    ///     isahc::get_async("http://localhost").await.expect_status_success();
    ///
    ///     surf::get("http://localhost").await.expect_status_success();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_success();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_success();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_success();
    /// }
    /// ```
    fn expect_status_success(&mut self) -> &mut T {
        self.expect_status_in_range(200, 300)
    }

    /// Expects response status to be in 3xx range
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_redirection();
    ///     reqwest::get("http://localhost").await.expect_status_redirection();
    ///
    ///     isahc::get("http://localhost").expect_status_redirection();
    ///     isahc::get_async("http://localhost").await.expect_status_redirection();
    ///
    ///     surf::get("http://localhost").await.expect_status_redirection();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_redirection();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_redirection();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_redirection();
    /// }
    /// ```
    fn expect_status_redirection(&mut self) -> &mut T {
        self.expect_status_in_range(300, 400)
    }

    /// Expects response status to be in 4xx range
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_client_error();
    ///     reqwest::get("http://localhost").await.expect_status_client_error();
    ///
    ///     isahc::get("http://localhost").expect_status_client_error();
    ///     isahc::get_async("http://localhost").await.expect_status_client_error();
    ///
    ///     surf::get("http://localhost").await.expect_status_client_error();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_client_error();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_client_error();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_client_error();
    /// }
    /// ```
    fn expect_status_client_error(&mut self) -> &mut T {
        self.expect_status_in_range(400, 500)
    }

    /// Expects response status to be in 5xx range
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_server_error();
    ///     reqwest::get("http://localhost").await.expect_status_server_error();
    ///
    ///     isahc::get("http://localhost").expect_status_server_error();
    ///     isahc::get_async("http://localhost").await.expect_status_server_error();
    ///
    ///     surf::get("http://localhost").await.expect_status_server_error();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_server_error();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_server_error();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_server_error();
    /// }
    /// ```
    fn expect_status_server_error(&mut self) -> &mut T {
        self.expect_status_in_range(500, 600)
    }

    /// Expects response status to be `OK 200`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_ok();
    ///     reqwest::get("http://localhost").await.expect_status_ok();
    ///
    ///     isahc::get("http://localhost").expect_status_ok();
    ///     isahc::get_async("http://localhost").await.expect_status_ok();
    ///
    ///     surf::get("http://localhost").await.expect_status_ok();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_ok();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_ok();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_ok();
    /// }
    /// ```
    fn expect_status_ok(&mut self) -> &mut T {
        self.expect_status(200)
    }

    /// Expects response status to be `Created 201`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_created();
    ///     reqwest::get("http://localhost").await.expect_status_created();
    ///
    ///     isahc::get("http://localhost").expect_status_created();
    ///     isahc::get_async("http://localhost").await.expect_status_created();
    ///
    ///     surf::get("http://localhost").await.expect_status_created();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_created();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_created();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_created();
    /// }
    /// ```
    fn expect_status_created(&mut self) -> &mut T {
        self.expect_status(201)
    }

    /// Expects response status to be `Accepted 202`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_accepted();
    ///     reqwest::get("http://localhost").await.expect_status_accepted();
    ///
    ///     isahc::get("http://localhost").expect_status_accepted();
    ///     isahc::get_async("http://localhost").await.expect_status_accepted();
    ///
    ///     surf::get("http://localhost").await.expect_status_accepted();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_accepted();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_accepted();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_accepted();
    /// }
    /// ```
    fn expect_status_accepted(&mut self) -> &mut T {
        self.expect_status(202)
    }

    /// Expects response status to be `No Content 204`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_no_content();
    ///     reqwest::get("http://localhost").await.expect_status_no_content();
    ///
    ///     isahc::get("http://localhost").expect_status_no_content();
    ///     isahc::get_async("http://localhost").await.expect_status_no_content();
    ///
    ///     surf::get("http://localhost").await.expect_status_no_content();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_no_content();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_no_content();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_no_content();
    /// }
    /// ```
    fn expect_status_no_content(&mut self) -> &mut T {
        self.expect_status(204)
    }

    /// Expects response status to be `Partial Content 206`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_partial_content();
    ///     reqwest::get("http://localhost").await.expect_status_partial_content();
    ///
    ///     isahc::get("http://localhost").expect_status_partial_content();
    ///     isahc::get_async("http://localhost").await.expect_status_partial_content();
    ///
    ///     surf::get("http://localhost").await.expect_status_partial_content();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_partial_content();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_partial_content();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_partial_content();
    /// }
    /// ```
    fn expect_status_partial_content(&mut self) -> &mut T {
        self.expect_status(206)
    }

    /// Expects response status to be `Bad Request 400`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_bad_request();
    ///     reqwest::get("http://localhost").await.expect_status_bad_request();
    ///
    ///     isahc::get("http://localhost").expect_status_bad_request();
    ///     isahc::get_async("http://localhost").await.expect_status_bad_request();
    ///
    ///     surf::get("http://localhost").await.expect_status_bad_request();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_bad_request();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_bad_request();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_bad_request();
    /// }
    /// ```
    fn expect_status_bad_request(&mut self) -> &mut T {
        self.expect_status(400)
    }

    /// Expects response status to be `Unauthorized 401`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_unauthorized();
    ///     reqwest::get("http://localhost").await.expect_status_unauthorized();
    ///
    ///     isahc::get("http://localhost").expect_status_unauthorized();
    ///     isahc::get_async("http://localhost").await.expect_status_unauthorized();
    ///
    ///     surf::get("http://localhost").await.expect_status_unauthorized();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_unauthorized();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_unauthorized();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_unauthorized();
    /// }
    /// ```
    fn expect_status_unauthorized(&mut self) -> &mut T {
        self.expect_status(401)
    }

    /// Expects response status to be `Forbidden 403`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_forbidden();
    ///     reqwest::get("http://localhost").await.expect_status_forbidden();
    ///
    ///     isahc::get("http://localhost").expect_status_forbidden();
    ///     isahc::get_async("http://localhost").await.expect_status_forbidden();
    ///
    ///     surf::get("http://localhost").await.expect_status_forbidden();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_forbidden();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_forbidden();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_forbidden();
    /// }
    /// ```
    fn expect_status_forbidden(&mut self) -> &mut T {
        self.expect_status(403)
    }

    /// Expects response status to be `Not Found 404`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_not_found();
    ///     reqwest::get("http://localhost").await.expect_status_not_found();
    ///
    ///     isahc::get("http://localhost").expect_status_not_found();
    ///     isahc::get_async("http://localhost").await.expect_status_not_found();
    ///
    ///     surf::get("http://localhost").await.expect_status_not_found();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_not_found();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_not_found();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_not_found();
    /// }
    /// ```
    fn expect_status_not_found(&mut self) -> &mut T {
        self.expect_status(404)
    }

    /// Expects response status to be `Conflict 409`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_conflict();
    ///     reqwest::get("http://localhost").await.expect_status_conflict();
    ///
    ///     isahc::get("http://localhost").expect_status_conflict();
    ///     isahc::get_async("http://localhost").await.expect_status_conflict();
    ///
    ///     surf::get("http://localhost").await.expect_status_conflict();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_conflict();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_conflict();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_conflict();
    /// }
    /// ```
    fn expect_status_conflict(&mut self) -> &mut T {
        self.expect_status(409)
    }

    /// Expects response status to be `Gone 410`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_gone();
    ///     reqwest::get("http://localhost").await.expect_status_gone();
    ///
    ///     isahc::get("http://localhost").expect_status_gone();
    ///     isahc::get_async("http://localhost").await.expect_status_gone();
    ///
    ///     surf::get("http://localhost").await.expect_status_gone();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_gone();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_gone();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_gone();
    /// }
    /// ```
    fn expect_status_gone(&mut self) -> &mut T {
        self.expect_status(410)
    }

    /// Expects response status to be `Internal Server Error 500`
    ///
    /// # Example
    /// ```no_run
    /// # use ureq::OrAnyStatus;
    /// use asserhttp::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     reqwest::blocking::get("http://localhost").expect_status_internal_server_error();
    ///     reqwest::get("http://localhost").await.expect_status_internal_server_error();
    ///
    ///     isahc::get("http://localhost").expect_status_internal_server_error();
    ///     isahc::get_async("http://localhost").await.expect_status_internal_server_error();
    ///
    ///     surf::get("http://localhost").await.expect_status_internal_server_error();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().expect_status_internal_server_error();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.expect_status_internal_server_error();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.expect_status_internal_server_error();
    /// }
    /// ```
    fn expect_status_internal_server_error(&mut self) -> &mut T {
        self.expect_status(500)
    }
}

impl<T> AsserhttpStatus<T> for T
where
    T: super::accessor::StatusAccessor,
{
    fn expect_status(&mut self, status: impl Into<AnyStatus>) -> &mut T {
        status.into().0(self.get_status());
        self
    }
}

impl<T, E> AsserhttpStatus<T> for Result<T, E>
where
    T: super::accessor::StatusAccessor,
    E: std::fmt::Debug,
{
    fn expect_status(&mut self, status: impl Into<AnyStatus>) -> &mut T {
        self.as_mut().unwrap().expect_status(status)
    }
}

pub struct AnyStatus(Box<dyn Fn(u16)>);

impl From<u16> for AnyStatus {
    fn from(expected: u16) -> Self {
        Self(Box::new(move |status| {
            assert_eq!(expected, status, "expected status to be '{}' but was '{}'", expected, status);
        }))
    }
}

impl From<crate::Status> for AnyStatus {
    fn from(value: crate::Status) -> Self {
        Self::from(u16::from_str(&value.to_string()).unwrap())
    }
}

#[cfg(feature = "rocket")]
impl From<rocket::http::Status> for AnyStatus {
    fn from(value: rocket::http::Status) -> Self {
        Self::from(value.code)
    }
}

impl<F: 'static> From<F> for AnyStatus
where
    F: Fn(u16),
{
    fn from(fun: F) -> Self {
        Self(Box::new(fun))
    }
}
