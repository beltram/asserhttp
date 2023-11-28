use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};
use crate::header::key::HeaderKey;
use crate::{AsserhttpError, AsserhttpResult};

type RocketResponse<'a> = rocket::local::blocking::LocalResponse<'a>;
type AsyncRocketResponse<'a> = rocket::local::asynchronous::LocalResponse<'a>;

impl<'a> StatusAccessor for RocketResponse<'a> {
    fn get_status(&self) -> u16 {
        self.status().code
    }
}

impl<'a> StatusAccessor for AsyncRocketResponse<'a> {
    fn get_status(&self) -> u16 {
        self.status().code
    }
}

impl<'a> HeaderAccessor for RocketResponse<'a> {
    fn get_keys(&self) -> Vec<HeaderKey> {
        self.headers().iter().map(|k| k.name.as_str().into()).collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &HeaderKey) -> Vec<String> {
        self.headers().get(key.as_ref()).map(str::to_string).collect::<Vec<_>>()
    }
}

impl<'a> HeaderAccessor for AsyncRocketResponse<'a> {
    fn get_keys(&self) -> Vec<HeaderKey> {
        self.headers().iter().map(|k| k.name.as_str().into()).collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &HeaderKey) -> Vec<String> {
        self.headers().get(key.as_ref()).map(str::to_string).collect::<Vec<_>>()
    }
}

impl<'a> BodyAccessor for RocketResponse<'a> {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];
        use std::io::Read as _;
        self.read_to_end(&mut buf).map_err(AsserhttpError::from)?;
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)
    }
}

impl<'a> BodyAccessor for AsyncRocketResponse<'a> {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];

        use rocket::tokio::io::AsyncReadExt as _;
        futures_lite::future::block_on(self.read_to_end(&mut buf)).map_err(AsserhttpError::from)?;
        if buf.is_empty() {
            return Err(AsserhttpError::BodyAbsent);
        }
        Ok(buf)
    }
}
