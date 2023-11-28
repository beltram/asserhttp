use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};
use crate::{header::key::HeaderKey, AsserhttpError, AsserhttpResult};

type ReqwestResponse = reqwest::blocking::Response;
type AsyncReqwestResponse = reqwest::Response;

impl StatusAccessor for ReqwestResponse {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl StatusAccessor for AsyncReqwestResponse {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl HeaderAccessor for ReqwestResponse {
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

impl HeaderAccessor for AsyncReqwestResponse {
    fn get_keys(&self) -> Vec<HeaderKey> {
        self.headers().keys().map(|k| k.as_str().into()).collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &HeaderKey) -> Vec<String> {
        let value = self
            .headers()
            .get(key.as_ref())
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string())
            .unwrap();
        vec![value]
    }
}

impl BodyAccessor for ReqwestResponse {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        let mut buf = vec![];
        self.copy_to(&mut buf)?;
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)
    }
}

impl BodyAccessor for AsyncReqwestResponse {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];
        while let Ok(Some(chunk)) = futures_lite::future::block_on(self.chunk()) {
            chunk.into_iter().for_each(|b| buf.push(b));
        }
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)
    }
}
