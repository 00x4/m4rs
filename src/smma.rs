//! SMMA (Smoothed Moving Average)
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
//! // Get 20SMMA calculation result
//! let result = m4rs::smma(&candlesticks, 20);
//! ```

use crate::{Error, IndexEntry, IndexEntryLike};

/// Returns SMMA (Smoothed Moving Average) for given IndexEntry list
pub fn smma(entries: &[impl IndexEntryLike], duration: usize) -> Result<Vec<IndexEntry>, Error> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    IndexEntry::validate_list(entries)?;
    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());

    let d = duration as f64;
    let mut smma = vec![];
    let mut last_smma = (0..duration).fold(0.0, |z, i| z + sorted[i].get_value()) / d;

    smma.push(IndexEntry {
        at: sorted[duration - 1].get_at(),
        value: last_smma,
    });

    for x in sorted.iter().skip(duration) {
        last_smma = (last_smma * (d - 1.0) + x.get_value()) / d;
        smma.push(IndexEntry {
            at: x.get_at(),
            value: last_smma,
        });
    }

    Ok(smma)
}
