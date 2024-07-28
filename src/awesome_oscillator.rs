//! Awesome Oscillator
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
//! // Get AO calculation result
//! let result = m4rs::awesome_oscillator(&candlesticks, 5, 34);
//! ```

use crate::{sma, Candlestick, Error, IndexEntry};

/// Returns Awesome Oscillator for given Candlestick list
pub fn awesome_oscillator(
    entries: &[Candlestick],
    short_duration: usize,
    long_duration: usize,
) -> Result<Vec<IndexEntry>, Box<dyn std::error::Error>> {
    if short_duration > long_duration {
        return Err(Box::new(Error::LongDurationIsNotGreaterThanShortDuration {
            short_duration,
            long_duration,
        }));
    }
    if short_duration == 0
        || long_duration == 0
        || long_duration == short_duration
        || entries.len() < short_duration
    {
        return Ok(vec![]);
    }
    Candlestick::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by(|a, b| a.at.cmp(&b.at));

    let median_prices: Vec<IndexEntry> = sorted
        .iter()
        .map(|x| IndexEntry {
            at: x.at,
            value: (x.high + x.low) * 0.5,
        })
        .collect();

    Ok(median_prices
        .iter()
        .filter_map(|x| {
            match (
                sma(&median_prices, short_duration)
                    .unwrap()
                    .iter()
                    .find(|ma| ma.at == x.at),
                sma(&median_prices, long_duration)
                    .unwrap()
                    .iter()
                    .find(|ma| ma.at == x.at),
            ) {
                (Some(short_ma), Some(long_ma)) => Some(IndexEntry {
                    at: x.at,
                    value: short_ma.value - long_ma.value,
                }),
                _ => None,
            }
        })
        .collect())
}
