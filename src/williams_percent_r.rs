//! Williams %R
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
//! // Get Williams %R calculation result
//! let result = m4rs::williams_percent_r(&candlesticks, 14);
//! ```

use crate::{Candlestick, IndexEntry};

/// Returns Williams %R for given Candlestick list
pub fn williams_percent_r(entries: &[Candlestick], duration: usize) -> Vec<IndexEntry> {
    if duration == 0 || entries.len() < duration {
        return vec![];
    }
    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.at);

    (0..=sorted.len() - duration)
        .map(|i| {
            let xs = sorted.iter().skip(i).take(duration);
            let highest = xs.clone().map(|x| x.high).reduce(|z, x| z.max(x)).unwrap();
            let lowest = xs.clone().map(|x| x.low).reduce(|z, x| z.min(x)).unwrap();
            let n = highest - lowest;
            let last = xs.last().unwrap();
            IndexEntry {
                at: last.at,
                value: if n == 0.0 {
                    0.0
                } else {
                    ((last.close - highest) / n) * 100.0
                },
            }
        })
        .collect()
}
