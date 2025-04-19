//! LWMA (Linear Weighted Moving Average)
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
//! // Get 20LWMA calculation result
//! let result = m4rs::lwma(&candlesticks, 20);
//! ```

use crate::{Error, IndexEntry, IndexEntryLike};

/// Returns LWMA (Linear Weighted Moving Average) for given IndexEntry list
pub fn lwma(entries: &[impl IndexEntryLike], duration: usize) -> Result<Vec<IndexEntry>, Error> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    IndexEntry::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());

    let d = duration as f64;
    let weight_sum: f64 = d * (d + 1.0) / 2.0;

    Ok((0..=(sorted.len() - duration))
        .map(|i| {
            let xs = sorted.iter().skip(i).take(duration);
            let weighted_sum: f64 = xs
                .enumerate()
                .map(|(j, x)| x.get_value() * ((j + 1) as f64))
                .sum();

            IndexEntry {
                at: sorted[i + duration - 1].get_at(),
                value: weighted_sum / weight_sum,
            }
        })
        .collect())
}
