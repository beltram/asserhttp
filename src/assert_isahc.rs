use crate::header::key::HeaderKey;
use crate::{AsserhttpError, AsserhttpResult};
use serde::de::DeserializeOwned;

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
        let mut buf = vec![];
        use isahc::ReadResponseExt as _;
        self.copy_to(&mut buf).map(|_| buf).map_err(AsserhttpError::from)
    }

    fn get_text(&mut self) -> AsserhttpResult<String> {
        use isahc::ReadResponseExt as _;
        self.text().map_err(AsserhttpError::from)
    }

    fn get_json<B>(&mut self) -> AsserhttpResult<B>
    where
        B: DeserializeOwned + Unpin,
    {
        use isahc::ReadResponseExt as _;
        Ok(self.json::<B>()?)
    }
}

impl BodyAccessor for AsyncIsahcResponse {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        let mut buf = vec![];
        use isahc::AsyncReadResponseExt as _;
        futures_lite::future::block_on(self.copy_to(&mut buf))
            .map(|_| buf)
            .map_err(AsserhttpError::from)
    }

    fn get_text(&mut self) -> AsserhttpResult<String> {
        use isahc::AsyncReadResponseExt as _;
        futures_lite::future::block_on(self.text()).map_err(AsserhttpError::from)
    }

    fn get_json<B>(&mut self) -> AsserhttpResult<B>
    where
        B: DeserializeOwned + Unpin,
    {
        use isahc::AsyncReadResponseExt as _;
        Ok(futures_lite::future::block_on(self.json::<B>())?)
    }
}
