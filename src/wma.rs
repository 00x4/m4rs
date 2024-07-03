//! Weighted Moving Average
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
//! // Get 20WMA calculation result
//! let result = m4rs::wma(&candlesticks, 20);
//! ```

use super::{IndexEntry, IndexEntryLike};

/// Returns WMA (Weighted Moving Average) for given IndexEntry list
pub fn wma(entries: &[impl IndexEntryLike], duration: usize) -> Vec<IndexEntry> {
    if duration == 0 || entries.len() < duration {
        return vec![];
    }

    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());

    (0..=(sorted.len() - duration))
        .map(|i| sorted.iter().skip(i).take(duration))
        .map(|xs| {
            let at = xs.clone().last().unwrap().get_at();
            let weights: Vec<f64> = (1..=duration).map(|x| x as f64).collect();
            let weights_sum = weights.iter().fold(0.0, |z, x| z + x);
            let value_sum = xs
                .zip(weights)
                .map(|(x, w)| x.get_value() * w)
                .fold(0.0, |z, x| z + x);
            IndexEntry {
                at,
                value: value_sum / weights_sum,
            }
        })
        .collect()
}
