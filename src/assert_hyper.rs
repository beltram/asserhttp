use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};
use crate::{header::key::HeaderKey, AsserhttpError, AsserhttpResult};
use http_body_util::BodyExt;

// type HyperResponse<B: http_body::Body> = hyper::Response<B>;

impl<B: http_body::Body> StatusAccessor for hyper::Response<B> {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl<B: http_body::Body> HeaderAccessor for hyper::Response<B> {
    fn get_keys(&self) -> Vec<HeaderKey> {
        self.headers().iter().map(|(k, _)| k.as_str().into()).collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &HeaderKey) -> Vec<String> {
        let value = self
            .headers()
            .get(key.as_ref())
            .and_then(|v| v.to_str().ok())
            .map(str::to_string)
            .unwrap();
        vec![value]
    }
}

impl<B> BodyAccessor for hyper::Response<B>
where
    B: http_body::Body + Sync,
    B: std::marker::Send,
    <B as http_body::Body>::Data: Send,
    AsserhttpError: From<<B as http_body::Body>::Error>,
    <B as http_body::Body>::Error: std::fmt::Debug,
    // <B as http_body::Body>::Error: std::fmt::Display,
{
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        use futures_lite::future::FutureExt as _;
        use http_body_util::BodyExt as _;
        // let buf = futures_lite::future::block_on(self.body_mut().collect())?.to_bytes().to_vec();
        let buf = futures_lite::future::block_on(self.body_mut().boxed().collect())
            .unwrap()
            .to_bytes()
            .to_vec();
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)
        /*use http_body_util::BodyExt as _;

        let mut buf = vec![];

        while let Some(Ok(frame)) = futures_lite::future::block_on(self.body_mut().frame()) {
            let data = frame.into_data().unwrap();
            buf.extend(data);
        }

        let buf = futures_lite::future::block_on(self.body_mut().collect())?.to_bytes().to_vec();
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)*/
    }
}
