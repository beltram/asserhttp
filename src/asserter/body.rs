use std::fmt::Debug;

use serde::de::DeserializeOwned;

pub const EMPTY_BODY_JSON_MSG: &str = "expected a json body but no response body was present";
pub const EMPTY_BODY_TEXT_MSG: &str = "expected a text body but no response body was present";
pub const EMPTY_BODY_BYTES_MSG: &str = "expected a response body but no response body was present";

pub fn assert_json_body_eq<B>(actual: B, expected: B) where B: DeserializeOwned + PartialEq + Debug + Unpin {
    assert_eq!(actual, expected, "expected json body {:?} to be equal to {:?} but was not", actual, expected);
}

pub fn assert_text_body(actual: String, expected: String) {
    assert_eq!(actual, expected, "expected text body '{}' to be equal to '{}' but was not", actual, expected);
}

pub fn assert_bytes_body(actual: &[u8], expected: &[u8]) {
    assert_eq!(actual, expected, "expected body '{:?}' to be equal to '{:?}' but was not", actual, expected);
}