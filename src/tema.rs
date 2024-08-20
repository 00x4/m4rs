//! TEMA (Triple Exponential Moving Average)
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
//! // Get 9TEMA calculation result
//! let result = m4rs::tema(&candlesticks, 9);
//! ```

use crate::{ema, Error, IndexEntry, IndexEntryLike};

/// Returns TEMA (Triple Exponential Moving Average) for given IndexEntry list
pub fn tema(entries: &[impl IndexEntryLike], duration: usize) -> Result<Vec<IndexEntry>, Error> {
    let ema1 = ema(entries, duration)?;
    let ema2 = ema(&ema1, duration)?;
    let ema3 = ema(&ema2, duration)?;
    Ok(ema3
        .iter()
        .filter_map(|e3| {
            match (
                ema1.iter().find(|x| x.at == e3.at),
                ema2.iter().find(|x| x.at == e3.at),
            ) {
                (Some(e1), Some(e2)) => Some(IndexEntry {
                    at: e1.at,
                    value: e1.value * 3.0 - e2.value * 3.0 + e3.value,
                }),
                _ => None,
            }
        })
        .collect())
}
