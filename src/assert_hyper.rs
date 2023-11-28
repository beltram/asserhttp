use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};
use crate::header::key::HeaderKey;
use crate::{AsserhttpError, AsserhttpResult};

type HyperResponse = hyper::Response<hyper::Body>;

impl StatusAccessor for HyperResponse {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl HeaderAccessor for HyperResponse {
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

impl BodyAccessor for HyperResponse {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];
        use hyper::body::HttpBody as _;
        while let Some(Ok(chunk)) = futures_lite::future::block_on(self.body_mut().data()) {
            chunk.into_iter().for_each(|b| buf.push(b));
        }
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)
    }
}
