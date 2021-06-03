pub fn assert_status(actual: u16, expected: u16) {
    assert_eq!(expected, actual, "expected status to be '{}' but was '{}'", expected, actual);
}

pub fn assert_status_range(actual: u16, expected_lower: u16, expected_upper: u16) {
    assert!(actual.ge(&expected_lower) && actual.lt(&expected_upper),
            "expected status to be in [{};{}[ but was '{}'", expected_lower, expected_upper, actual);
}