use serde::{Deserialize, Serialize};

pub mod surf;
pub mod reqwest;
pub mod isahc;
pub mod hyper;
pub mod awc;
pub mod actix;
pub mod rocket;
pub mod utils;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct Body {
    pub a: String,
}