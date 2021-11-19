use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct Body {
    pub a: String,
}
