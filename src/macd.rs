//! MACD (Moving Average Convergence Divergence)
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
//! // Get MACD calculation result
//! let result = m4rs::macd(&candlesticks, 12, 26, 9);
//! ```

use std::fmt::Display;

use super::ema;
use super::IndexEntryLike;

#[derive(Debug, Clone)]
pub struct MacdEntry {
    pub at: u64,
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

impl Display for MacdEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MACD(at={} macd={} sig={} hist={})",
            self.at, self.macd, self.signal, self.histogram
        )
    }
}

impl IndexEntryLike for MacdEntry {
    fn get_at(&self) -> u64 {
        self.at
    }

    fn get_value(&self) -> f64 {
        self.macd
    }
}

/// Returns MACD for given IndexEntry list
pub fn macd(
    entries: &[impl IndexEntryLike],
    short: usize,
    long: usize,
    signal: usize,
) -> Vec<MacdEntry> {
    if short == 0 || long == 0 || signal == 0 {
        return vec![];
    }
    let ema_s = ema(entries, short);
    let ema_l = ema(entries, long);
    let macds: Vec<MacdEntry> = ema_l
        .iter()
        .filter_map(|l| {
            ema_s.iter().find(|s| s.at == l.at).map(|s| MacdEntry {
                at: s.at,
                macd: s.value - l.value,
                signal: 0.0,
                histogram: 0.0,
            })
        })
        .collect();
    let signals = ema(&macds, signal);
    macds
        .iter()
        .filter_map(|x| {
            signals
                .iter()
                .find(|s| s.at == x.at)
                .map(|signal| MacdEntry {
                    at: x.at,
                    macd: x.macd,
                    signal: signal.value,
                    histogram: x.macd - signal.value,
                })
        })
        .collect()
}
