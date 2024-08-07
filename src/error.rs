use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Error {
    ContainsNaN {
        at: u64,
        field: String,
    },
    ContainsInfinite {
        at: u64,
        field: String,
    },
    LongDurationIsNotGreaterThanShortDuration {
        short_duration: usize,
        long_duration: usize,
    },
    MustBePositiveF32 {
        value: f32,
        field: String,
    },
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
