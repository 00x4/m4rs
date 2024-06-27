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
