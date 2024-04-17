use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};
use crate::{header::key::HeaderKey, AsserhttpError, AsserhttpResult};

type AxumResponse = axum::response::Response;

impl StatusAccessor for AxumResponse {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl HeaderAccessor for AxumResponse {
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

impl BodyAccessor for AxumResponse {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        use http_body_util::BodyExt as _;
        let buf = futures_lite::future::block_on(self.body_mut().collect())?.to_bytes().to_vec();
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)
    }
}
