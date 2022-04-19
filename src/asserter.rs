use std::fmt::{Debug, Display};

use regex::Regex;
use serde::{de::DeserializeOwned, Serialize};

pub const EMPTY_BODY_JSON_MSG: &str = "expected a json body but no response body was present";
pub const EMPTY_BODY_TEXT_MSG: &str = "expected a text body but no response body was present";
pub const EMPTY_BODY_BYTES_MSG: &str = "expected a response body but no response body was present";
pub const EXPECTED_BODY_PRESENT_MSG: &str = "expected a response body but no response body was present";
pub const EXPECTED_BODY_ABSENT_MSG: &str = "expected no response body but a response body was present";

pub fn assert_status(actual: u16, expected: u16) {
    assert_eq!(expected, actual, "expected status to be '{}' but was '{}'", expected, actual);
}

pub fn assert_status_range(actual: u16, expected_lower: u16, expected_upper: u16) {
    assert!(actual.ge(&expected_lower) && actual.lt(&expected_upper),
            "expected status to be in [{};{}[ but was '{}'", expected_lower, expected_upper, actual);
}

pub fn assert_header_key<T, I>(actual_keys: T, expected: I)
    where T: Iterator<Item=I>,
          I: AsRef<str> + PartialEq + Display {
    let expected = expected.as_ref().to_lowercase();
    let mut actual_keys = actual_keys.map(|k| k.as_ref().to_lowercase());
    assert!(actual_keys.any(|k| k == expected),
            "expected one header named '{}' but none found", expected);
}

pub fn assert_header_value<T, I>(actual_values: T, key: I, expected: I)
    where T: Iterator<Item=I>,
          I: AsRef<str> + PartialEq + Display + Debug {
    let actual_values = actual_values.collect::<Vec<I>>();
    let count = actual_values.len();
    assert_eq!(count, 1, "expected header '{}' to be single valued. Had '{}' values '{:?}'. Use 'expect_headers' instead.", key, count, actual_values);
    let value = actual_values.first().unwrap();
    assert_eq!(value, &expected, "expected header '{}' to be equal to '{}' but was '{}'", key, expected, value);
}

pub fn assert_header_values(actual_values: Vec<String>, key: &str, expected: Vec<String>) {
    assert!(!expected.is_empty(), "no value expected for header '{}'. Use 'expect_header_present' instead", key);
    let same_size = actual_values.len() == expected.len();
    let all_eq = actual_values.iter()
        .zip(expected.iter())
        .all(|(a, b)| a == b);
    assert!(same_size && all_eq, "expected header '{}' to contain values '{:?}' but contained '{:?}'", key, expected, actual_values)
}

pub fn assert_header_key_absent<T, I>(actual_keys: T, expected: I)
    where T: Iterator<Item=I>,
          I: Into<String> + PartialEq + Display {
    let expected = expected.into().to_lowercase();
    let mut actual_keys = actual_keys.map(|k| k.into().to_lowercase());
    assert!(!actual_keys.any(|k| k == expected),
            "expected no header named '{}' but one found", expected);
}

pub fn assert_json_body_eq<B>(actual: B, expected: B) where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin {
    assert_eq!(actual, expected, "expected json body '{}' to be equal to '{}' but was not",
               to_json_string(&actual),
               to_json_string(&expected));
}

fn to_json_string<B>(json: &B) -> String where B: DeserializeOwned + Serialize + PartialEq + Debug + Unpin {
    serde_json::to_string(json)
        .unwrap_or_else(|_| String::from("Failed serializing actual response body"))
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