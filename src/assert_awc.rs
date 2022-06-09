use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};

type AwcResponse = awc::ClientResponse<actix_http::BoxedPayloadStream>;

impl StatusAccessor for AwcResponse {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl HeaderAccessor for AwcResponse {
    fn get_keys(&self) -> Vec<String> {
        self.headers().iter()
            .map(|(k, _)| k.as_str().to_string())
            .collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &str) -> Vec<String> {
        let value = self.headers().get(key)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string)
            .unwrap();
        vec![value]
    }
}

impl BodyAccessor for AwcResponse {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        futures_lite::future::block_on(self.body())
            .map(|b| b.to_vec())
            .map_err(anyhow::Error::msg)
    }
}