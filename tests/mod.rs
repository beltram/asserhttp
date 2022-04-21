use serde::{Deserialize, Serialize};

pub mod http_client;
pub mod actix;
pub mod rocket;
pub mod utils;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct Body {
    pub a: String,
}