use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    ContainsNaN(u64),
    ContainsInfinite(u64),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
