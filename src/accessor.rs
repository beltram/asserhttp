use serde::de::DeserializeOwned;

pub trait StatusAccessor {
    fn get_status(&self) -> u16;
}

pub trait HeaderAccessor {
    fn get_keys(&self) -> Vec<String>;

    fn get_raw_values(&self, key: &str) -> Vec<String>;

    fn get_values(&self, key: &str) -> Vec<String> {
        self.get_raw_values(key).iter()
            .flat_map(|v| v.split(',').map(str::trim))
            .map(str::to_string)
            .collect::<Vec<_>>()
    }
}

pub trait BodyAccessor {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>>;

    fn get_text(&mut self) -> anyhow::Result<String> {
        std::str::from_utf8(self.get_bytes()?.as_slice())
            .map_err(anyhow::Error::msg)
            .map(str::to_string)
    }

    fn get_json<B>(&mut self) -> anyhow::Result<B> where B: DeserializeOwned + Unpin {
        serde_json::from_slice::<B>(self.get_bytes()?.as_slice())
            .map_err(anyhow::Error::msg)
    }
}