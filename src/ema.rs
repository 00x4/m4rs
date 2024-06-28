//! Exponential Moving Average
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
//! // Get 20EMA calculation result
//! let result = m4rs::ema(&candlesticks, 20);
//! ```

use super::{IndexEntry, IndexEntryLike};

/// Returns EMA (Exponential Moving Average) for given IndexEntry list
pub fn ema(entries: &[impl IndexEntryLike], duration: usize) -> Vec<IndexEntry> {
    ema_with_alpha(entries, duration, 2.0 / ((duration as f64) + 1.0))
}

pub(crate) fn ema_with_alpha<T: IndexEntryLike>(
    entries: &[T],
    duration: usize,
    alpha: f64,
) -> Vec<IndexEntry> {
    if duration == 0 || entries.len() < duration {
        return vec![];
    }

    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());

    let first_ma = {
        let xs: Vec<&T> = sorted.iter().take(duration).collect();
        IndexEntry {
            at: xs.last().unwrap().get_at(),
            value: xs.iter().fold(0.0, |z, x| z + x.get_value()) / (xs.len() as f64),
        }
    };

    sorted
        .iter()
        .skip(duration)
        .scan(first_ma, |z, x| {
            z.at = x.get_at();
            z.value = z.value + alpha * (x.get_value() - z.value);
            Some(z.clone())
        })
        .collect()
}
