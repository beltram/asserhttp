use std::fmt::Debug;

use serde::de::DeserializeOwned;

pub fn assert_json_body<B>(actual: anyhow::Result<B>, expected: B) where B: DeserializeOwned + PartialEq + Debug + Unpin {
    if let Ok(actual) = actual {
        assert_eq!(actual, expected, "expected json body {:?} to be equal to {:?} but was not", actual, expected);
    } else {
        panic!("expected json body {:?} but no response body was present", expected)
    }
}