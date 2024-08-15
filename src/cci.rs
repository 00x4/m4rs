//! CCI (Commodity Channel Index)
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
//! // Get CCI calculation result
//! let result = m4rs::cci(&candlesticks, 14);
//! ```

use crate::{sma, Candlestick, Error, IndexEntry, IndexEntryLike};

/// Returns CCI for given Candlestick list
pub fn cci(entries: &[Candlestick], duration: usize) -> Result<Vec<IndexEntry>, Error> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    Candlestick::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by(|a, b| a.at.cmp(&b.at));

    let tp: Vec<IndexEntry> = sorted.iter().map(|x| x.to_typical_price_entry()).collect();
    let ma = sma(&tp, duration)?;

    Ok(average_deviations(&tp, duration)
        .iter()
        .filter_map(|md| {
            match (
                tp.iter().find(|x| x.at == md.at),
                ma.iter().find(|x| x.at == md.at),
            ) {
                (Some(tp), Some(ma)) => Some(IndexEntry {
                    at: md.at,
                    value: (tp.value - ma.value) / (md.value * 0.015),
                }),
                _ => None,
            }
        })
        .collect())
}

fn average_deviations(xs: &[IndexEntry], duration: usize) -> Vec<IndexEntry> {
    (0..=(xs.len() - duration))
        .map(|i| xs.iter().skip(i).take(duration))
        .map(|xs| IndexEntry {
            at: xs.clone().last().unwrap().at,
            value: average_deviation(&xs.map(|x| x.get_value()).collect::<Vec<f64>>()),
        })
        .collect()
}

fn average_deviation(xs: &[f64]) -> f64 {
    if xs.is_empty() {
        return 0.0;
    }
    let size = xs.len() as f64;
    let avg = xs.iter().fold(0.0, |z, x| z + x) / size;
    xs.iter().map(|x| (x - avg).abs()).fold(0.0, |z, x| z + x) / size
}
