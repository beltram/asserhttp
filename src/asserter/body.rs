use std::fmt::Debug;

use serde::de::DeserializeOwned;

pub fn assert_json_body_eq<B>(actual: B, expected: B) where B: DeserializeOwned + PartialEq + Debug + Unpin {
    assert_eq!(actual, expected, "expected json body {:?} to be equal to {:?} but was not", actual, expected);
}

pub fn assert_text_body(actual: anyhow::Result<String>, expected: String) {
    if let Ok(actual) = actual {
        if !actual.is_empty() {
            assert_eq!(actual, expected, "expected text body '{}' to be equal to '{}' but was not", actual, expected);
        } else {
            panic!("expected text body '{}' but no response body was present", expected)
        }
    } else {
        panic!("expected text body '{}' but no response body was present", expected)
    }
}