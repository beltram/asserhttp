use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};

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
    fn get_keys(&self) -> Vec<String> {
        self.headers().keys().map(|k| k.to_string()).collect::<Vec<_>>()
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

impl HeaderAccessor for AsyncReqwestResponse {
    fn get_keys(&self) -> Vec<String> {
        self.headers().keys().map(|k| k.to_string()).collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &str) -> Vec<String> {
        let value = self
            .headers()
            .get(key)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string())
            .unwrap();
        vec![value]
    }
}

impl BodyAccessor for ReqwestResponse {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut buf = vec![];
        self.copy_to(&mut buf).map(|_| buf).map_err(anyhow::Error::msg)
    }
}

impl BodyAccessor for AsyncReqwestResponse {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];
        while let Ok(Some(chunk)) = futures_lite::future::block_on(self.chunk()) {
            chunk.into_iter().for_each(|b| buf.push(b));
        }
        Ok(buf)
    }
}
