#[derive(Debug, Clone)]
pub struct HeaderValues(Vec<String>);

impl<S: AsRef<str>> From<Vec<S>> for HeaderValues {
    fn from(values: Vec<S>) -> Self {
        let values = values.into_iter().map(|v| v.as_ref().to_string()).collect();
        Self(values)
    }
}

impl<const N: usize, S: AsRef<str>> From<[S; N]> for HeaderValues {
    fn from(values: [S; N]) -> Self {
        values.map(|v| v.as_ref().to_string()).to_vec().into()
    }
}

impl<'a, const N: usize, S: AsRef<str>> From<&'a [S; N]> for HeaderValues {
    fn from(values: &'a [S; N]) -> Self {
        values.iter().map(|v| v.as_ref().to_string()).collect::<Vec<_>>().into()
    }
}

impl From<http_types::headers::HeaderValues> for HeaderValues {
    fn from(values: http_types::headers::HeaderValues) -> Self {
        values.iter().map(|v| v.to_string()).collect::<Vec<String>>().into()
    }
}

impl std::ops::Deref for HeaderValues {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for HeaderValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0.as_slice())
    }
}

impl Eq for HeaderValues {}

impl PartialEq for HeaderValues {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}
