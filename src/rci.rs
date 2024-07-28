//! RCI (Rank Correlation Index)
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
//! // Get RCI calculation result
//! let result = m4rs::rci(&candlesticks, 9);
//! ```

use std::cmp::Ordering;

use crate::{IndexEntry, IndexEntryLike};

/// Returns RCI for given IndexEntry list
pub fn rci(
    entries: &[impl IndexEntryLike],
    duration: usize,
) -> Result<Vec<IndexEntry>, Box<dyn std::error::Error>> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    IndexEntry::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());

    Ok((0..=sorted.len() - duration)
        .map(|i| {
            let xs: Vec<_> = sorted.iter().skip(i).take(duration).collect();
            let last = xs.last().unwrap();
            let date_ranked: Vec<_> = xs.iter().rev().collect();
            let price_ranked = {
                let mut xs = xs.clone();
                xs.sort_by(|a, b| {
                    if a.get_value() > b.get_value() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                xs
            };

            let d = xs
                .iter()
                .filter_map(|x| {
                    match (
                        date_ranked.iter().position(|d| x.get_at() == d.get_at()),
                        price_ranked.iter().position(|p| x.get_at() == p.get_at()),
                    ) {
                        (Some(date_rank), Some(price_rank)) => {
                            let n = date_rank as f64 - price_rank as f64;
                            Some(n * n)
                        }
                        _ => None,
                    }
                })
                .fold(0.0, |z, x| z + x);

            let duration = duration as f64;
            IndexEntry {
                at: last.get_at(),
                value: (1.0 - (6.0 * d) / (duration.powi(3) - duration)) * 100.0,
            }
        })
        .collect())
}
