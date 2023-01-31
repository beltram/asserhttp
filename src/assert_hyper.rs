use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};

type HyperResponse = hyper::Response<hyper::Body>;

impl StatusAccessor for HyperResponse {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl HeaderAccessor for HyperResponse {
    fn get_keys(&self) -> Vec<String> {
        self.headers().iter().map(|(k, _)| k.as_str().to_string()).collect::<Vec<_>>()
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

impl BodyAccessor for HyperResponse {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];
        use hyper::body::HttpBody as _;
        while let Some(Ok(chunk)) = futures_lite::future::block_on(self.body_mut().data()) {
            chunk.into_iter().for_each(|b| buf.push(b));
        }
        Ok(buf)
    }
}
