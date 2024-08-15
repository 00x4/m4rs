//! RMA (Running Moving Average)
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
//! // Get 20RMA calculation result
//! let result = m4rs::rma(&candlesticks, 20);
//! ```

use crate::{ema_with_alpha, Error, IndexEntry, IndexEntryLike};

/// Returns RMA (Running Moving Average) for given IndexEntry list
pub fn rma(entries: &[impl IndexEntryLike], duration: usize) -> Result<Vec<IndexEntry>, Error> {
    ema_with_alpha(entries, duration, 1.0 / (duration as f64))
}
