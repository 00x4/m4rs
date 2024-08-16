//! Standard Deviation
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
//! // Get Standard Deviation calculation result
//! let result = m4rs::standard_deviation(&candlesticks, 20);
//! ```

use crate::{Error, IndexEntry, IndexEntryLike};

/// Returns Standard Deviation for given IndexEntry list
pub fn standard_deviation(
    entries: &[impl IndexEntryLike],
    duration: usize,
) -> Result<Vec<IndexEntry>, Error> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    IndexEntry::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());

    Ok((0..=(sorted.len() - duration))
        .map(|i| sorted.iter().skip(i).take(duration))
        .map(|xs| {
            let d = duration as f64;
            let avg = xs.clone().fold(0.0, |z, x| z + x.get_value()) / d;
            let value = (xs
                .clone()
                .fold(0.0, |z, x| z + (x.get_value() - avg).abs().powi(2))
                / d)
                .sqrt();
            IndexEntry {
                at: xs.last().unwrap().get_at(),
                value,
            }
        })
        .collect())
}
