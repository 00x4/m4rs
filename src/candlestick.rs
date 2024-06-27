//! Candlestick (OHLCV) data object

use std::fmt::Display;

use super::{IndexEntry, IndexEntryLike};

/// Candlestick entry
#[derive(Debug, Clone)]
pub struct Candlestick {
    pub at: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl Display for Candlestick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Candlestick(at={} o={} h={} l={} c={} v={})",
            self.at, self.open, self.high, self.low, self.close, self.volume
        )
    }
}

impl IndexEntryLike for Candlestick {
    fn get_at(&self) -> u64 {
        self.at
    }

    fn get_value(&self) -> f64 {
        self.close
    }
}

impl Candlestick {
    /// Creates new Candlestick instance
    pub fn new(at: u64, open: f64, high: f64, low: f64, close: f64, volume: f64) -> Candlestick {
        Candlestick {
            at,
            open,
            high,
            low,
            close,
            volume,
        }
    }

    /// Converts to IndexEntry with value field as volume
    pub fn to_volume_entry(&self) -> IndexEntry {
        IndexEntry {
            at: self.at,
            value: self.volume,
        }
    }

    /// Calculates typical price
    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }

    /// Converts to IndexEntry with value field as typical price
    pub fn to_typical_price_entry(&self) -> IndexEntry {
        IndexEntry {
            at: self.at,
            value: self.typical_price(),
        }
    }

    /// Returns true if bullish (white candlestick)
    pub fn is_bullish(&self) -> bool {
        self.open < self.close
    }

    /// Returns true if bearish (black candlestick)
    pub fn is_bearish(&self) -> bool {
        self.open > self.close
    }

    /// Returns body length
    pub fn body_size(&self) -> f64 {
        (self.open - self.close).abs()
    }

    /// Returns highest value in open and close prices
    pub fn body_high(&self) -> f64 {
        self.open.max(self.close)
    }

    /// Returns lowest value in open and close prices
    pub fn body_low(&self) -> f64 {
        self.open.min(self.close)
    }

    /// Returns upper shadow length
    pub fn upper_shadow_size(&self) -> f64 {
        self.high - self.open.max(self.close)
    }

    /// Returns lower shadow length
    pub fn lower_shadow_size(&self) -> f64 {
        self.open.min(self.close) - self.low
    }
}

#[cfg(test)]
mod tests {
    use crate::IndexEntryLike;

    #[test]
    fn test_candlestick_to_volume_entry() {
        let c1 = super::Candlestick::new(1001, 10.0, 20.0, 5.0, 9.0, 123.0);
        assert_eq!(1001, c1.get_at());
        assert_eq!(9.0, c1.get_value());

        let got = c1.to_volume_entry();
        assert_eq!(1001, got.get_at());
        assert_eq!(123.0, got.get_value());
    }

    #[test]
    fn test_candlestick_typical_price() {
        let h = 20.0;
        let l = 5.0;
        let c = 9.0;
        let c1 = super::Candlestick::new(1001, 10.0, h, l, c, 123.0);
        assert_eq!((h + l + c) / 3.0, c1.typical_price());
    }

    #[test]
    fn test_candlestick_to_typical_price_entry() {
        let h = 20.0;
        let l = 5.0;
        let c = 9.0;
        let c1 = super::Candlestick::new(1001, 10.0, h, l, c, 123.0);
        assert_eq!(1001, c1.get_at());
        assert_eq!(c, c1.get_value());

        let got = c1.to_typical_price_entry();
        assert_eq!(1001, got.get_at());
        assert_eq!((h + l + c) / 3.0, got.get_value());
    }

    #[test]
    fn test_candlestick_is_bullish_or_bearish() {
        let c1 = super::Candlestick::new(1001, 10.0, 20.0, 5.0, 15.0, 123.0);
        assert!(c1.is_bullish());
        assert!(!c1.is_bearish());

        let c2 = super::Candlestick::new(1001, 10.0, 20.0, 5.0, 5.0, 123.0);
        assert!(!c2.is_bullish());
        assert!(c2.is_bearish());

        let c3 = super::Candlestick::new(1001, 10.0, 20.0, 5.0, 10.0, 123.0);
        assert!(!c3.is_bullish());
        assert!(!c3.is_bearish());
    }

    #[test]
    fn test_candlestick_body() {
        let c1 = super::Candlestick::new(1001, 10.0, 20.0, 5.0, 15.0, 123.0);
        assert_eq!(5.0, c1.body_size());
        assert_eq!(15.0, c1.body_high());
        assert_eq!(10.0, c1.body_low());

        let c2 = super::Candlestick::new(1001, 10.0, 20.0, 5.0, 6.0, 123.0);
        assert_eq!(4.0, c2.body_size());
        assert_eq!(10.0, c2.body_high());
        assert_eq!(6.0, c2.body_low());

        let c3 = super::Candlestick::new(1001, 10.0, 20.0, 5.0, 10.0, 123.0);
        assert_eq!(0.0, c3.body_size());
        assert_eq!(10.0, c3.body_high());
        assert_eq!(10.0, c3.body_low());
    }

    #[test]
    fn test_candlestick_shadow() {
        let c1 = super::Candlestick::new(1001, 10.0, 20.0, 5.0, 15.0, 123.0);
        assert_eq!(5.0, c1.upper_shadow_size());
        assert_eq!(5.0, c1.lower_shadow_size());

        let c2 = super::Candlestick::new(1001, 10.0, 20.0, 5.0, 6.0, 123.0);
        assert_eq!(10.0, c2.upper_shadow_size());
        assert_eq!(1.0, c2.lower_shadow_size());

        let c3 = super::Candlestick::new(1001, 10.0, 20.0, 5.0, 10.0, 123.0);
        assert_eq!(10.0, c3.upper_shadow_size());
        assert_eq!(5.0, c3.lower_shadow_size());

        let c4 = super::Candlestick::new(1001, 10.0, 10.0, 5.0, 10.0, 123.0);
        assert_eq!(0.0, c4.upper_shadow_size());
        assert_eq!(5.0, c4.lower_shadow_size());

        let c5 = super::Candlestick::new(1001, 10.0, 20.0, 10.0, 10.0, 123.0);
        assert_eq!(10.0, c5.upper_shadow_size());
        assert_eq!(0.0, c5.lower_shadow_size());
    }
}
