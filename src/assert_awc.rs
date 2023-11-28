use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};
use crate::{header::key::HeaderKey, AsserhttpError, AsserhttpResult};

type AwcResponse = awc::ClientResponse<actix_http::BoxedPayloadStream>;

impl StatusAccessor for AwcResponse {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl HeaderAccessor for AwcResponse {
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

impl BodyAccessor for AwcResponse {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        let buf = futures_lite::future::block_on(self.body()).map(|b| b.to_vec())?;
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)
    }
}
