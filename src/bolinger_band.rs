//! Bolinger Band
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
//! // Get Bolinger Band calculation result
//! let result = m4rs::bolinger_band(&candlesticks, 20);
//! ```

use std::fmt::Display;

use crate::{IndexEntry, IndexEntryLike};

#[derive(Clone, Debug)]
pub struct BollingerBandEntry {
    pub at: u64,
    pub avg: f64,
    pub sigma: f64,
}

impl BollingerBandEntry {
    pub fn upper_sigma(&self, weight: f32) -> f64 {
        self.avg + self.sigma * weight as f64
    }

    pub fn lower_sigma(&self, weight: f32) -> f64 {
        self.avg - self.sigma * weight as f64
    }

    pub fn upper_sigma1(&self) -> f64 {
        self.upper_sigma(1.0)
    }

    pub fn upper_sigma2(&self) -> f64 {
        self.upper_sigma(2.0)
    }

    pub fn upper_sigma3(&self) -> f64 {
        self.upper_sigma(3.0)
    }

    pub fn lower_sigma1(&self) -> f64 {
        self.lower_sigma(1.0)
    }

    pub fn lower_sigma2(&self) -> f64 {
        self.lower_sigma(2.0)
    }

    pub fn lower_sigma3(&self) -> f64 {
        self.lower_sigma(3.0)
    }
}

impl Display for BollingerBandEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BolingerBand(at={} avg={} sigma={})",
            self.at, self.avg, self.sigma,
        )
    }
}

impl IndexEntryLike for BollingerBandEntry {
    fn get_at(&self) -> u64 {
        self.at
    }

    fn get_value(&self) -> f64 {
        self.avg
    }
}

/// Returns Bolinger Band for given Candlestick list
pub fn bolinger_band(
    entries: &[impl IndexEntryLike],
    duration: usize,
) -> Result<Vec<BollingerBandEntry>, Box<dyn std::error::Error>> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    IndexEntry::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());

    Ok((0..sorted.len() - duration + 1)
        .map(|i| {
            let xs = sorted.iter().skip(i).take(duration);
            let d = duration as f64;
            let avg = xs.clone().fold(0.0, |z, x| z + x.get_value()) / d;
            let sigma = xs
                .clone()
                .fold(0.0, |z, x| z + (x.get_value() - avg).powi(2) / d)
                .sqrt();
            BollingerBandEntry {
                at: xs.last().unwrap().get_at(),
                avg,
                sigma,
            }
        })
        .collect())
}
