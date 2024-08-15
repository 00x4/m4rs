//! Heikin Ahi
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
//! // Get Heikin Ashi calculation result
//! let result = m4rs::heikin_ashi(&candlesticks);
//! ```

use crate::{Candlestick, Error};

pub fn heikin_ashi(entries: &[Candlestick]) -> Result<Vec<Candlestick>, Error> {
    Candlestick::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by(|a, b| a.at.cmp(&b.at));

    Ok(sorted.iter().fold(vec![], |z, x| {
        if z.is_empty() {
            return vec![x.clone()];
        }
        let last = z.last().unwrap();
        [
            z.clone(),
            vec![Candlestick {
                at: x.at,
                open: (last.open + last.close) / 2.0,
                high: x.high,
                low: x.low,
                close: (x.open + x.close + x.high + x.low) / 4.0,
                volume: x.volume,
            }],
        ]
        .concat()
    }))
}
