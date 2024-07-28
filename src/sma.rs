//! SMA (Simple Moving Average)
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
//! // Get 20SMA calculation result
//! let result = m4rs::sma(&candlesticks, 20);
//! ```

use super::{IndexEntry, IndexEntryLike};

/// Returns SMA (Simple Moving Average) for given IndexEntry list
pub fn sma(
    entries: &[impl IndexEntryLike],
    duration: usize,
) -> Result<Vec<IndexEntry>, Box<dyn std::error::Error>> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    IndexEntry::validate_list(entries)?;
    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());
    Ok((0..=(sorted.len() - duration))
        .map(|i| sorted.iter().skip(i).take(duration))
        .map(|xs| IndexEntry {
            at: xs.clone().last().unwrap().get_at(),
            value: xs.fold(0.0, |z, x| z + x.get_value()) / (duration as f64),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use crate::Candlestick;

    #[test]
    fn test_sma() {
        let xs = vec![
            Candlestick::new(1719400001, 100.0, 130.0, 90.0, 110.0, 1000.0),
            Candlestick::new(1719400002, 110.0, 140.0, 100.0, 130.0, 1000.0),
            Candlestick::new(1719400003, 130.0, 135.0, 120.0, 120.0, 1000.0),
            Candlestick::new(1719400004, 120.0, 130.0, 80.0, 95.0, 1000.0),
            Candlestick::new(1719400005, 90.0, 100.0, 70.0, 82.0, 1000.0),
        ];
        let got = super::sma(&xs, 3);
        assert!(got.is_ok());
        let got = got.unwrap();
        assert_eq!(3, got.len());
        assert_eq!(1719400003, got[0].at);
        assert_eq!(120.0, got[0].value);
        assert_eq!(1719400004, got[1].at);
        assert_eq!(115.0, got[1].value);
        assert_eq!(1719400005, got[2].at);
        assert_eq!(99.0, got[2].value);
    }
}
