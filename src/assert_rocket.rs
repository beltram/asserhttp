use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};

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
    fn get_keys(&self) -> Vec<String> {
        self.headers().iter().map(|k| k.name.to_string()).collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &str) -> Vec<String> {
        self.headers().get(key).map(str::to_string).collect::<Vec<_>>()
    }
}

impl<'a> HeaderAccessor for AsyncRocketResponse<'a> {
    fn get_keys(&self) -> Vec<String> {
        self.headers().iter().map(|k| k.name.to_string()).collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &str) -> Vec<String> {
        self.headers().get(key.as_ref()).map(str::to_string).collect::<Vec<_>>()
    }
}

impl<'a> BodyAccessor for RocketResponse<'a> {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];
        use std::io::Read as _;
        self.read_to_end(&mut buf).map(|_| buf).map_err(anyhow::Error::msg)
    }
}

impl<'a> BodyAccessor for AsyncRocketResponse<'a> {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];

        use rocket::tokio::io::AsyncReadExt as _;
        futures_lite::future::block_on(self.read_to_end(&mut buf))
            .map(|_| buf)
            .map_err(anyhow::Error::msg)
    }
}
