use crate::header::key::HeaderKey;
use crate::{AsserhttpError, AsserhttpResult};

use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};

type SurfResponse = surf::Response;

impl StatusAccessor for SurfResponse {
    fn get_status(&self) -> u16 {
        u16::from(self.status())
    }
}

impl HeaderAccessor for SurfResponse {
    fn get_keys(&self) -> Vec<HeaderKey> {
        self.header_names().map(|k| k.as_str().into()).collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &HeaderKey) -> Vec<String> {
        self.header(key.as_ref())
            .map(|values| values.into_iter().map(|v| v.to_string()).collect::<Vec<_>>())
            .unwrap_or_default()
    }
}

impl BodyAccessor for SurfResponse {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        let buf = futures_lite::future::block_on(self.body_bytes()).map_err(AsserhttpError::from)?;
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)
    }
}
