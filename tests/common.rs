use serde::Deserialize;

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Body {
    pub a: String,
}
