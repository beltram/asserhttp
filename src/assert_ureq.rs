use std::io::Read;

use itertools::*;

use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};

type UreqResponse = ureq::Response;

impl StatusAccessor for UreqResponse {
    fn get_status(&self) -> u16 {
        self.status()
    }
}

impl HeaderAccessor for UreqResponse {
    fn get_keys(&self) -> Vec<String> {
        self.headers_names()
    }

    fn get_raw_values(&self, key: &str) -> Vec<String> {
        self.header(key)
            .map(|v| vec![v.to_string()])
            .unwrap_or_default()
    }
}

impl BodyAccessor for UreqResponse {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        let headers = self.headers_names().iter()
        .filter_map(|k| self.header(&k).map(|v| (k, v)))
        .map(|(k, v)| format!("{}: {}", k, v))
        .join("\r\n");
        let headers = format!("{}\r\n{}", self.status_text(), headers);
        let mut resp_cpy = UreqResponse::new(self.status(), &headers, "").unwrap();
        std::mem::swap(self, &mut resp_cpy);
        let mut buf: Vec<u8> = vec![];
        resp_cpy.into_reader()
            .read_to_end(&mut buf)
            .map(|_| buf)
            .map_err(anyhow::Error::msg)
    }
}

