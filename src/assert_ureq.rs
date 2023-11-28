use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};
use crate::header::key::HeaderKey;
use crate::{AsserhttpError, AsserhttpResult};

type UreqResponse = ureq::Response;

impl StatusAccessor for UreqResponse {
    fn get_status(&self) -> u16 {
        self.status()
    }
}

impl HeaderAccessor for UreqResponse {
    fn get_keys(&self) -> Vec<HeaderKey> {
        self.headers_names().into_iter().map(|k| k.into()).collect()
    }

    fn get_raw_values(&self, key: &HeaderKey) -> Vec<String> {
        self.header(key.as_ref()).map(|v| vec![v.to_string()]).unwrap_or_default()
    }
}

impl BodyAccessor for UreqResponse {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        use itertools::Itertools as _;
        let headers = self
            .headers_names()
            .iter()
            .filter_map(|k| self.header(k).map(|v| (k, v)))
            .map(|(k, v)| format!("{k}: {v}"))
            .join("\r\n");
        let headers = format!("{}\r\n{}", self.status_text(), headers);
        let mut resp_cpy = UreqResponse::new(self.status(), &headers, "").unwrap();
        std::mem::swap(self, &mut resp_cpy);
        let mut buf: Vec<u8> = vec![];
        use std::io::Read as _;
        resp_cpy.into_reader().read_to_end(&mut buf).map_err(AsserhttpError::from)?;
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)
    }
}
