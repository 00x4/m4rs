//! Momentum
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
//! // Get Momentum calculation result
//! let result = m4rs::momentum(&candlesticks, 10);
//! ```

use super::{IndexEntry, IndexEntryLike};

/// Returns Momentum for given IndexEntry list
pub fn momentum(entries: &[impl IndexEntryLike], duration: usize) -> Vec<IndexEntry> {
    if duration == 0 || entries.len() < duration {
        return vec![];
    }
    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());
    (0..(sorted.len() - duration))
        .map(|i| sorted.iter().skip(i).take(duration + 1))
        .map(|mut xs| {
            let head = xs.next().unwrap();
            let last = xs.last().unwrap();
            IndexEntry {
                at: last.get_at(),
                value: last.get_value() - head.get_value(),
            }
        })
        .collect()
}
