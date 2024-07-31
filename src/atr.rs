//! ATR (Average True Range)
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
//! // Get ATR calculation result
//! let result = m4rs::atr(&candlesticks, 14);
//! ```

use super::rma;
use super::Candlestick;
use super::IndexEntry;

/// Returns ATR (Average True Range) for given Candlestick list
pub fn atr(
    entries: &[Candlestick],
    duration: usize,
) -> Result<Vec<IndexEntry>, Box<dyn std::error::Error>> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    Candlestick::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by(|a, b| a.at.cmp(&b.at));

    let mut tr: Vec<IndexEntry> = vec![];
    for (i, x) in sorted.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let prev = sorted.get(i - 1).unwrap();
        let r1 = x.high - prev.close;
        let r2 = (x.low - prev.close).abs();
        let r3 = (x.high - x.low).abs();
        tr.push(IndexEntry {
            at: x.at,
            value: r1.max(r2).max(r3),
        });
    }
    rma(&tr, duration)
}
