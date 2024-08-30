//! Williams Fractals
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
//! // Get Fractals calculation result
//! let result = m4rs::williams_fractals(&candlesticks, 2);
//! ```

use std::fmt::Display;

use crate::{Candlestick, Error};

#[derive(Clone, Debug)]
pub struct WilliamsFractalsEntry {
    pub at: u64,
    pub up: bool,
    pub down: bool,
}

impl Display for WilliamsFractalsEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Fractals(at={} up={} down={})",
            self.at, self.up, self.down
        )
    }
}

/// Returns Williams Fractals for given Candlestick list
pub fn williams_fractals(
    entries: &[Candlestick],
    duration: usize,
) -> Result<Vec<WilliamsFractalsEntry>, Error> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    Candlestick::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by(|a, b| a.at.cmp(&b.at));

    let take_range = duration * 2 + 1;
    let ret: Vec<WilliamsFractalsEntry> = (0..=(sorted.len() - take_range))
        .map(|i| sorted.iter().skip(i).take(take_range))
        .map(|xs| {
            let mid = xs.clone().nth(duration).unwrap();
            let init = xs.clone().take(duration);
            let tail = xs.clone().skip(duration + 1).take(duration);
            let up = init
                .clone()
                .map(|x| x.high)
                .reduce(|z, x| z.max(x))
                .unwrap()
                < mid.high
                && tail
                    .clone()
                    .map(|x| x.high)
                    .reduce(|z, x| z.max(x))
                    .unwrap()
                    < mid.high;
            let down = init.clone().map(|x| x.low).reduce(|z, x| z.min(x)).unwrap() > mid.low
                && tail.clone().map(|x| x.low).reduce(|z, x| z.min(x)).unwrap() > mid.low;
            WilliamsFractalsEntry {
                at: mid.at,
                up,
                down,
            }
        })
        .collect();

    let rest = sorted
        .iter()
        .rev()
        .take(duration)
        .rev()
        .map(|x| WilliamsFractalsEntry {
            at: x.at,
            up: false,
            down: false,
        })
        .collect();

    Ok([ret, rest].concat())
}
