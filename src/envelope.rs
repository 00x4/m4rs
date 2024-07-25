//! Envelope
//!
//! # Examples
//! ```rust
//! // Prepare candlesticks in some way
//! let candlesticks = vec![
//!     m4rs::Candlestick::new(1719400001, 100.0, 130.0, 90.0, 110.0, 1000.0),
//!     m4rs::Candlestick::new(1719400002, 110.0, 140.0, 100.0, 130.0, 1000.0),
//!     m4rs::Candlestick::new(1719400003, 130.0, 135.0, 120.0, 120.0, 1000.0),
//!     m4rs::Candlestick::new(1719400004, 120.0, 130.0, 80.0, 95.0, 1000.0),
//!     m4rs::Candlestick::new(1719400005, 90.0, 100.0, 70.0, 82.0, 1000.0),
//! ];
//!
//! // Get 20SMA
//! let ma = m4rs::sma(&candlesticks, 20);
//!
//! // Get Envelope with 10% range
//! let result = m4rs::envelope(&ma, 10.0);
//! ```

use std::fmt::Display;

use crate::IndexEntryLike;

#[derive(Clone, Debug)]
pub struct EnvelopeEntry {
    at: u64,
    basis: f64,
    upper: f64,
    lower: f64,
}

impl Display for EnvelopeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Envelope(at={} basis={} upper={} lower={})",
            self.at, self.basis, self.upper, self.lower,
        )
    }
}

impl IndexEntryLike for EnvelopeEntry {
    fn get_at(&self) -> u64 {
        self.at
    }

    fn get_value(&self) -> f64 {
        self.basis
    }
}

/// Returns Envelope for given IndexEntry list
pub fn envelope(entries: &[impl IndexEntryLike], percent: f32) -> Vec<EnvelopeEntry> {
    entries
        .iter()
        .map(|x| {
            let basis = x.get_value();
            let pct = percent as f64 / 100.0;
            EnvelopeEntry {
                at: x.get_at(),
                basis,
                upper: basis * (1.0 + pct),
                lower: basis * (1.0 - pct),
            }
        })
        .collect()
}
