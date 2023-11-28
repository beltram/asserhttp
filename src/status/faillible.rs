use crate::accessor::StatusAccessor;
use crate::error::{AsserhttpError, AsserhttpResult};

/// For assertions on http response status
pub trait FaillibleAsserhttpStatus<T> {
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
    fn try_expect_status(&mut self, status: impl Into<FallibleAnyStatus>) -> AsserhttpResult<&mut T>;
}

impl<T> FaillibleAsserhttpStatus<T> for T
where
    T: StatusAccessor,
{
    fn try_expect_status(&mut self, status: impl Into<FallibleAnyStatus>) -> AsserhttpResult<&mut T> {
        status.into()(self.get_status())?;
        Ok(self)
    }
}

impl<T, E> FaillibleAsserhttpStatus<T> for Result<T, E>
where
    T: StatusAccessor,
    E: std::fmt::Debug,
{
    fn try_expect_status(&mut self, status: impl Into<FallibleAnyStatus>) -> AsserhttpResult<&mut T> {
        self.as_mut()
            .map_err(|e| AsserhttpError::HttpError(format!("{e:?}")))?
            .try_expect_status(status)
    }
}

pub struct FallibleAnyStatus(Box<dyn Fn(u16) -> AsserhttpResult<()>>);

impl From<u16> for FallibleAnyStatus {
    fn from(expected: u16) -> Self {
        Self(Box::new(move |actual| {
            if actual != expected {
                return Err(AsserhttpError::StatusMismatch { expected, actual });
            }
            Ok(())
        }))
    }
}

impl From<crate::Status> for FallibleAnyStatus {
    fn from(value: crate::Status) -> Self {
        value.to_string().parse::<u16>().map(Self::from).unwrap()
    }
}

impl<F: 'static> From<F> for FallibleAnyStatus
where
    F: Fn(u16) -> AsserhttpResult<()>,
{
    fn from(fun: F) -> Self {
        Self(Box::new(fun))
    }
}

#[cfg(feature = "rocket")]
impl From<rocket::http::Status> for FallibleAnyStatus {
    fn from(value: rocket::http::Status) -> Self {
        Self::from(value.code)
    }
}

impl std::ops::Deref for FallibleAnyStatus {
    type Target = dyn Fn(u16) -> AsserhttpResult<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
