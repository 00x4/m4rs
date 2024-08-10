//! RSI (Relative Strength Index)
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
//! // Get RSI calculation result
//! let result = m4rs::rsi(&candlesticks, 14);
//! ```

use crate::{Error, IndexEntry, IndexEntryLike};

#[derive(Clone)]
struct Calc {
    result: f64,
    prev: IndexEntry,
    upside: f64,
    downside: f64,
}

/// Returns RSI for given IndexEntry list
pub fn rsi<T: IndexEntryLike>(entries: &[T], duration: usize) -> Result<Vec<IndexEntry>, Error> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    IndexEntry::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());

    let first_rsi = calc_first_rsi(&sorted, duration);
    if first_rsi.is_none() {
        return Ok(vec![]);
    }
    let first_rsi = first_rsi.unwrap();
    let xs: Vec<&T> = sorted.iter().skip(duration + 1).collect();
    if xs.is_empty() {
        return Ok(vec![IndexEntry {
            at: first_rsi.0,
            value: first_rsi.1.result,
        }]);
    }
    Ok(xs
        .iter()
        .scan(first_rsi, |z, x| {
            let upside = (z.1.upside * ((duration - 1) as f64)
                + (x.get_value() - z.1.prev.value).max(0.0))
                / (duration as f64);
            let downside = (z.1.downside * ((duration - 1) as f64)
                + (z.1.prev.value - x.get_value()).max(0.0))
                / (duration as f64);
            z.0 = x.get_at();
            z.1 = Calc {
                result: upside / (upside + downside) * 100.0,
                prev: IndexEntry::from(*x),
                upside,
                downside,
            };
            Some(z.clone())
        })
        .map(|(at, calc)| IndexEntry {
            at,
            value: calc.result,
        })
        .collect())
}

fn calc_first_rsi<T: IndexEntryLike>(entries: &[T], duration: usize) -> Option<(u64, Calc)> {
    if duration == 0 {
        return None;
    }
    let xs: Vec<&T> = entries.iter().take(duration + 1).collect();
    if xs.is_empty() || xs.len() < duration + 1 {
        return None;
    }

    let upside = xs
        .iter()
        .map(|x| x.get_value())
        .fold((-1.0, 0.0), |(z, a), b| {
            if z < 0.0 {
                (0.0, b)
            } else if a < b {
                (z + (b - a).abs(), b)
            } else {
                (z, b)
            }
        })
        .0
        / (duration as f64);

    let downside = xs
        .iter()
        .map(|x| x.get_value())
        .fold((-1.0, 0.0), |(z, a), b| {
            if z < 0.0 {
                (0.0, b)
            } else if a > b {
                (z + (b - a).abs(), b)
            } else {
                (z, b)
            }
        })
        .0
        / (duration as f64);

    let last = xs.last().unwrap();
    Some((
        xs.last().unwrap().get_at(),
        Calc {
            result: upside / (upside + downside) * 100.0,
            prev: IndexEntry::from(*last),
            upside,
            downside,
        },
    ))
}
