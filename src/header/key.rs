use crate::AsserhttpError;
use http_types::headers::HeaderName;

#[derive(Debug, Clone)]
pub struct HeaderKey(String);

impl<'a> From<&'a str> for HeaderKey {
    fn from(name: &'a str) -> Self {
        Self(name.to_string())
    }
}

impl<'a> From<&'a String> for HeaderKey {
    fn from(name: &'a String) -> Self {
        Self(name.to_string())
    }
}

impl From<String> for HeaderKey {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl From<HeaderName> for HeaderKey {
    fn from(name: HeaderName) -> Self {
        Self(name.as_str().to_string())
    }
}

impl std::ops::Deref for HeaderKey {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for HeaderKey {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for HeaderKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Eq for HeaderKey {}

impl PartialEq for HeaderKey {
    fn eq(&self, other: &Self) -> bool {
        self.eq_ignore_ascii_case(other)
    }
}

impl HeaderKey {
    pub fn try_assert_contained(&self, actual_keys: Vec<HeaderKey>) -> crate::AsserhttpResult<()> {
        if !actual_keys.contains(self) {
            return Err(AsserhttpError::HeaderAbsent { key: self.clone() });
        }
        Ok(())
    }

    pub fn try_assert_absent(&self, actual_keys: Vec<HeaderKey>) -> crate::AsserhttpResult<()> {
        if actual_keys.contains(self) {
            return Err(AsserhttpError::HeaderPresent { key: self.clone() });
        }
        Ok(())
    }
}
