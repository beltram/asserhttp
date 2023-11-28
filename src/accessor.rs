use crate::{
    header::{key::HeaderKey, values::HeaderValues},
    AsserhttpError, AsserhttpResult,
};
use serde::de::DeserializeOwned;

pub trait StatusAccessor {
    fn get_status(&self) -> u16;
}

pub trait HeaderAccessor {
    fn get_keys(&self) -> Vec<HeaderKey>;

    fn get_raw_values(&self, key: &HeaderKey) -> Vec<String>;

    fn get_values(&self, key: &HeaderKey) -> HeaderValues {
        self.get_raw_values(key)
            .iter()
            .flat_map(|v| v.split(',').map(str::trim))
            .map(str::to_string)
            .collect::<Vec<_>>()
            .into()
    }
}

pub trait BodyAccessor {
    fn get_bytes(&mut self) -> AsserhttpResult<Vec<u8>>;

    fn get_text(&mut self) -> AsserhttpResult<String> {
        std::str::from_utf8(self.get_bytes()?.as_slice())
            .map(str::to_string)
            .map_err(AsserhttpError::from)
    }

    fn get_json<B>(&mut self) -> AsserhttpResult<B>
    where
        B: DeserializeOwned + Unpin,
    {
        Ok(serde_json::from_slice::<B>(self.get_bytes()?.as_slice())?)
    }
}

pub trait AllAccessors: StatusAccessor + HeaderAccessor + BodyAccessor {}

impl<T> AllAccessors for T where T: StatusAccessor + HeaderAccessor + BodyAccessor {}
