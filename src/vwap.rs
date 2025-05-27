//! VWAP (Volume Weighted Average Price)
//!
//! # Examples
//! ```rust
//! // Prepare candlesticks in some way
//! let candlesticks = vec![
//!     m4rs::Candlestick::new(1719400001, 100.0, 130.0, 90.0, 110.0, 1000.0),
//!     m4rs::Candlestick::new(1719400002, 110.0, 140.0, 100.0, 130.0, 1500.0),
//!     m4rs::Candlestick::new(1719400003, 130.0, 135.0, 120.0, 120.0, 800.0),
//!     m4rs::Candlestick::new(1719400004, 120.0, 130.0, 80.0, 95.0, 1200.0),
//!     m4rs::Candlestick::new(1719400005, 90.0, 100.0, 70.0, 82.0, 900.0),
//! ];
//!
//! // Get VWAP calculation result
//! let result = m4rs::vwap(&candlesticks);
//! ```

use crate::{Candlestick, Error, IndexEntry};

/// Returns VWAP for given Candlestick list
pub fn vwap(entries: &[Candlestick]) -> Result<Vec<IndexEntry>, Error> {
    if entries.is_empty() {
        return Ok(vec![]);
    }

    Candlestick::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by(|a, b| a.at.cmp(&b.at));

    let mut cum_price_vol = 0.0;
    let mut cum_vol = 0.0;
    let mut result = Vec::new();

    for x in &sorted {
        if x.volume == 0.0 {
            continue;
        }

        cum_price_vol += x.typical_price() * x.volume;
        cum_vol += x.volume;

        result.push(IndexEntry {
            at: x.at,
            value: cum_price_vol / cum_vol,
        });
    }

    Ok(result)
}
