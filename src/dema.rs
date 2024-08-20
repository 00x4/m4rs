//! DEMA (Double Exponential Moving Average)
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
//! // Get 9DEMA calculation result
//! let result = m4rs::dema(&candlesticks, 9);
//! ```

use crate::{ema, Error, IndexEntry, IndexEntryLike};

/// Returns DEMA (Double Exponential Moving Average) for given IndexEntry list
pub fn dema(entries: &[impl IndexEntryLike], duration: usize) -> Result<Vec<IndexEntry>, Error> {
    let ema1 = ema(entries, duration)?;
    let ema2 = ema(&ema1, duration)?;
    Ok(ema2
        .iter()
        .filter_map(|e2| {
            ema1.iter().find(|e1| e1.at == e2.at).map(|e1| IndexEntry {
                at: e1.at,
                value: e1.value * 2.0 - e2.value,
            })
        })
        .collect())
}
