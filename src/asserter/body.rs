use std::fmt::Debug;

use regex::Regex;
use serde::de::DeserializeOwned;

pub const EMPTY_BODY_JSON_MSG: &str = "expected a json body but no response body was present";
pub const EMPTY_BODY_TEXT_MSG: &str = "expected a text body but no response body was present";
pub const INVALID_UTF8_BODY_TEXT_MSG: &str = "expected a text body but contained non utf-8 characters";
pub const EMPTY_BODY_BYTES_MSG: &str = "expected a response body but no response body was present";
pub const EXPECTED_BODY_PRESENT_MSG: &str = "expected a response body but no response body was present";
pub const EXPECTED_BODY_ABSENT_MSG: &str = "expected no response body but a response body was present";

pub fn assert_json_body_eq<B>(actual: B, expected: B) where B: DeserializeOwned + PartialEq + Debug + Unpin {
    assert_eq!(actual, expected, "expected json body {:?} to be equal to {:?} but was not", actual, expected);
}

pub fn assert_text_body(actual: String, expected: String) {
    assert_eq!(actual, expected, "expected text body '{}' to be equal to '{}' but was not", actual, expected);
}

pub fn assert_body_regex(actual: String, regex: Regex) {
    assert!(regex.is_match(actual.as_str()), "expected text body '{}' to match regex '{}' but did not", actual, regex);
}

pub fn assert_bytes_body(actual: &[u8], expected: &[u8]) {
    assert_eq!(actual, expected, "expected body '{:?}' to be equal to '{:?}' but was not", actual, expected);
}