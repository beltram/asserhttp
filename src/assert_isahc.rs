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
    fn get_keys(&self) -> Vec<String> {
        self.headers().keys().map(|k| k.as_str().to_string()).collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &str) -> Vec<String> {
        let value = self
            .headers()
            .get(key)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string)
            .unwrap();
        vec![value]
    }
}

impl HeaderAccessor for AsyncIsahcResponse {
    fn get_keys(&self) -> Vec<String> {
        self.headers().keys().map(|k| k.as_str().to_string()).collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &str) -> Vec<String> {
        let value = self
            .headers()
            .get(key)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string)
            .unwrap();
        vec![value]
    }
}

impl BodyAccessor for IsahcResponse {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut buf = vec![];
        use isahc::ReadResponseExt as _;
        self.copy_to(&mut buf).map(|_| buf).map_err(anyhow::Error::msg)
    }

    fn get_text(&mut self) -> anyhow::Result<String> {
        use isahc::ReadResponseExt as _;
        self.text().map_err(anyhow::Error::msg)
    }

    fn get_json<B>(&mut self) -> anyhow::Result<B>
    where
        B: DeserializeOwned + Unpin,
    {
        use isahc::ReadResponseExt as _;
        self.json::<B>().map_err(anyhow::Error::msg)
    }
}

impl BodyAccessor for AsyncIsahcResponse {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut buf = vec![];
        use isahc::AsyncReadResponseExt as _;
        futures_lite::future::block_on(self.copy_to(&mut buf))
            .map(|_| buf)
            .map_err(anyhow::Error::msg)
    }

    fn get_text(&mut self) -> anyhow::Result<String> {
        use isahc::AsyncReadResponseExt as _;
        futures_lite::future::block_on(self.text()).map_err(anyhow::Error::msg)
    }

    fn get_json<B>(&mut self) -> anyhow::Result<B>
    where
        B: DeserializeOwned + Unpin,
    {
        use isahc::AsyncReadResponseExt as _;
        futures_lite::future::block_on(self.json::<B>()).map_err(anyhow::Error::msg)
    }
}
