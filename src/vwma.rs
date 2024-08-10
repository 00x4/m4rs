//! VWMA (Volume Weighted Moving Average)
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
//! // Get VWMA calculation result
//! let result = m4rs::vwma(&candlesticks, 20);
//! ```

use crate::{Candlestick, Error, IndexEntry};

/// Returns VWMA (Volume Weighted Moving Average) for given Candlestick list
pub fn vwma(entries: &[Candlestick], duration: usize) -> Result<Vec<IndexEntry>, Error> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    Candlestick::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by(|a, b| a.at.cmp(&b.at));

    let res: Vec<IndexEntry> = (0..=(sorted.len() - duration))
        .map(|i| sorted.iter().skip(i).take(duration))
        .map(|xs| {
            let (cv, v) = xs.clone().fold((0.0, 0.0), |z, x| {
                (z.0 + x.close * x.volume, z.1 + x.volume)
            });
            IndexEntry {
                at: xs.last().unwrap().at,
                value: if v == 0.0 { f64::NAN } else { cv / v },
            }
        })
        .collect();

    match res.iter().find(|x| x.value.is_nan()) {
        Some(x) => Err(Error::DividedByZero {
            at: x.at,
            field: "sum of volume".to_string(),
        }),
        None => Ok(res),
    }
}
