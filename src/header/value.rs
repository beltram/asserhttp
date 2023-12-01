#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HeaderValue(String);

impl<'a> From<&'a str> for HeaderValue {
    fn from(value: &'a str) -> Self {
        Self(value.to_string())
    }
}

impl<'a> From<&'a String> for HeaderValue {
    fn from(value: &'a String) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for HeaderValue {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<http_types::headers::HeaderValue> for HeaderValue {
    fn from(value: http_types::headers::HeaderValue) -> Self {
        Self(value.as_str().to_string())
    }
}

impl std::ops::Deref for HeaderValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for HeaderValue {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
