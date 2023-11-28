use crate::{header::key::HeaderKey, AsserhttpError, AsserhttpResult};

use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};

type IsahcResponse = isahc::Response<isahc::Body>;
type AsyncIsahcResponse = isahc::Response<isahc::AsyncBody>;

impl StatusAccessor for IsahcResponse {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl StatusAccessor for AsyncIsahcResponse {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl HeaderAccessor for IsahcResponse {
    fn get_keys(&self) -> Vec<HeaderKey> {
        self.headers().keys().map(|k| k.as_str().into()).collect::<Vec<_>>()
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

impl HeaderAccessor for AsyncIsahcResponse {
    fn get_keys(&self) -> Vec<HeaderKey> {
        self.headers().keys().map(|k| k.as_str().into()).collect::<Vec<_>>()
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

impl BodyAccessor for IsahcResponse {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        use isahc::ReadResponseExt as _;
        let buf = self.bytes()?;
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)
    }
}

impl BodyAccessor for AsyncIsahcResponse {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        use isahc::AsyncReadResponseExt as _;
        let buf = futures_lite::future::block_on(self.bytes()).map_err(AsserhttpError::from)?;
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)
    }
}
