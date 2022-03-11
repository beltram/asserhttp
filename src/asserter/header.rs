use std::fmt::{Debug, Display};

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

pub fn assert_header_values<'a, T, I>(actual_values: T, key: I, expected: Vec<&'a str>)
    where T: Iterator<Item=I>,
          I: TryInto<&'a str> + PartialEq + Display + Debug {
    assert!(!expected.is_empty(), "no value expected for header '{}'. Use 'expect_header_present' instead", key);
    let actual_values = actual_values
        .filter_map(|it| it.try_into().ok())
        .collect::<Vec<&str>>();
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