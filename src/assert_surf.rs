use serde::de::DeserializeOwned;

use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};

type SurfResponse = surf::Response;

impl StatusAccessor for SurfResponse {
    fn get_status(&self) -> u16 {
        u16::from(self.status())
    }
}

impl HeaderAccessor for SurfResponse {
    fn get_keys(&self) -> Vec<String> {
        self.header_names()
            .map(|k| k.to_string())
            .collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &str) -> Vec<String> {
        self.header(key)
            .map(|values| {
                values.into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }
}

impl BodyAccessor for SurfResponse {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        futures_lite::future::block_on(self.body_bytes()).map_err(anyhow::Error::msg)
    }

    fn get_text(&mut self) -> anyhow::Result<String> {
        futures_lite::future::block_on(self.body_string()).map_err(anyhow::Error::msg)
    }

    fn get_json<B>(&mut self) -> anyhow::Result<B> where B: DeserializeOwned {
        futures_lite::future::block_on(self.body_json::<B>()).map_err(anyhow::Error::msg)
    }
}