use crate::{
    accessor::StatusAccessor,
    error::{AsserhttpError, AsserhttpResult},
    status::infallible::AnyStatus,
};

/// For assertions on http response status returning an error instead of panicking
pub trait FallibleAsserhttpStatus<T> {
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
    ///     reqwest::blocking::get("http://localhost").try_expect_status(200).unwrap();
    ///     reqwest::blocking::get("http://localhost").try_expect_status(Status::Ok).unwrap();
    ///     reqwest::blocking::get("http://localhost").try_expect_status(|s| { assert_eq!(s, 200); Ok(()) }).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_status(200).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_status(Status::Ok).unwrap();
    ///     reqwest::get("http://localhost").await.try_expect_status(|s| { assert_eq!(s, 200); Ok(()) }).unwrap();
    ///
    ///     isahc::get("http://localhost").try_expect_status(200).unwrap();
    ///     isahc::get("http://localhost").try_expect_status(Status::Ok).unwrap();
    ///     isahc::get("http://localhost").try_expect_status(|s| { assert_eq!(s, 200); Ok(()) }).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_status(200).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_status(Status::Ok).unwrap();
    ///     isahc::get_async("http://localhost").await.try_expect_status(|s| { assert_eq!(s, 200); Ok(()) }).unwrap();
    ///
    ///     surf::get("http://localhost").await.try_expect_status(200).unwrap();
    ///     surf::get("http://localhost").await.try_expect_status(Status::Ok).unwrap();
    ///     surf::get("http://localhost").await.try_expect_status(|s| { assert_eq!(s, 200); Ok(()) }).unwrap();
    ///
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_status(200).unwrap();
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_status(Status::Ok).unwrap();
    ///     ureq::get("http://localhost").call().or_any_status().try_expect_status(|s| { assert_eq!(s, 200); Ok(()) }).unwrap();
    ///
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_status(200).unwrap();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_status(Status::Ok).unwrap();
    ///     hyper::Client::new().get("http://localhost".parse().unwrap()).await.try_expect_status(|s| { assert_eq!(s, 200); Ok(()) }).unwrap();
    ///
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_status(200).unwrap();
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_status(Status::Ok).unwrap();
    ///     awc::Client::default().get("http://localhost").send().await.try_expect_status(|s| { assert_eq!(s, 200); Ok(()) }).unwrap();
    /// }
    /// ```
    ///
    /// # Error
    /// * [AsserhttpError::StatusMismatch] when actual status is different from expected one
    fn try_expect_status(&mut self, status: impl Into<FallibleAnyStatus>) -> AsserhttpResult<&mut T>;
}

impl<T> FallibleAsserhttpStatus<T> for T
where
    T: StatusAccessor,
{
    fn try_expect_status(&mut self, status: impl Into<FallibleAnyStatus>) -> AsserhttpResult<&mut T> {
        status.into()(self.get_status())?;
        Ok(self)
    }
}

impl<T, E> FallibleAsserhttpStatus<T> for Result<T, E>
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

impl From<AnyStatus> for FallibleAnyStatus {
    fn from(asserter: AnyStatus) -> Self {
        Self(Box::new(move |status| {
            asserter(status);
            Ok(())
        }))
    }
}
