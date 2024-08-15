//! HMA (Hull Moving Average)
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
//! // Get 20HMA calculation result
//! let result = m4rs::hma(&candlesticks, 20);
//! ```

use crate::{wma, Error, IndexEntry, IndexEntryLike};

/// Returns HMA (Hull Moving Average) for given IndexEntry list
pub fn hma(entries: &[impl IndexEntryLike], duration: usize) -> Result<Vec<IndexEntry>, Error> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    IndexEntry::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());
    let d = duration as f32;

    let wma_half = wma(&sorted, (d / 2.0) as usize)?;
    let raw: Vec<IndexEntry> = wma(&sorted, duration)?
        .iter()
        .filter_map(|f| {
            wma_half.iter().find(|h| h.at == f.at).map(|h| IndexEntry {
                at: h.at,
                value: h.value * 2.0 - f.value,
            })
        })
        .collect();
    wma(&raw, d.sqrt() as usize)
}
