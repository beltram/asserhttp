use std::{convert::TryInto, fmt::{Debug, Display}};

pub fn assert_header_key<'a, T, I>(mut actual_keys: T, expected: I) where T: Iterator<Item=I>, I: Into<&'a str> + PartialEq + Display {
    assert!(actual_keys.any(|k| k == expected), "expected one header named '{}' but none found", expected);
}

pub fn assert_header_value<'a, T, I>(actual_values: T, key: I, expected: I) where T: Iterator<Item=I>, I: TryInto<&'a str> + PartialEq + Display + Debug {
    let actual_values = actual_values.collect::<Vec<I>>();
    let count = actual_values.len();
    assert_eq!(count, 1, "expected header '{}' to be single valued. Had '{}' values '{:?}'. Use 'expect_headers' instead.", key, count, actual_values);
    let value = actual_values.first().unwrap();
    assert_eq!(value, &expected, "expected header '{}' to be equal to '{}' but was '{}'", key, &expected, value);
}