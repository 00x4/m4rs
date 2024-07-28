//! Stochastics
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
//! // Get Fast Stochastics calculation result
//! let result = m4rs::stochastics(&candlesticks, 14, 3);
//!
//! // Get Slow Stochastics calculation result
//! let result = m4rs::slow_stochastics(&candlesticks, 14, 3, 5);
//! ```

use std::fmt::Display;

use crate::{sma, Candlestick, IndexEntry};

use super::IndexEntryLike;

#[derive(Clone, Debug)]
pub struct StochasticsEntry {
    pub at: u64,
    /// %K
    pub k: f64,
    /// %D
    pub d: f64,
}

impl Display for StochasticsEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stochastics(at={} k={} d={})", self.at, self.k, self.d,)
    }
}

impl IndexEntryLike for StochasticsEntry {
    fn get_at(&self) -> u64 {
        self.at
    }

    fn get_value(&self) -> f64 {
        self.k
    }
}

#[derive(Clone, Debug)]
pub struct SlowStochasticsEntry {
    pub at: u64,
    /// %K
    pub k: f64,
    /// %D
    pub d: f64,
    /// Slow%D
    pub sd: f64,
}

impl SlowStochasticsEntry {
    fn with_k(&self, k: f64) -> SlowStochasticsEntry {
        SlowStochasticsEntry {
            at: self.at,
            k,
            d: self.d,
            sd: self.sd,
        }
    }

    fn with_d(&self, d: f64) -> SlowStochasticsEntry {
        SlowStochasticsEntry {
            at: self.at,
            k: self.k,
            d,
            sd: self.sd,
        }
    }
}

impl Display for SlowStochasticsEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Stochastics(at={} k={} d={} sd={})",
            self.at, self.k, self.d, self.sd,
        )
    }
}

impl IndexEntryLike for SlowStochasticsEntry {
    fn get_at(&self) -> u64 {
        self.at
    }

    fn get_value(&self) -> f64 {
        self.d
    }
}

/// Returns Stochastics for given Candlestick list
pub fn stochastics(
    entries: &[Candlestick],
    duration_k: usize,
    duration_d: usize,
) -> Result<Vec<StochasticsEntry>, Box<dyn std::error::Error>> {
    if duration_k == 0 || duration_d == 0 || entries.len() < duration_k {
        return Ok(vec![]);
    }
    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());
    let ks = calc_k(&sorted, duration_k);
    let ds = sma(&ks, duration_d)?;
    Ok(ds
        .iter()
        .filter_map(|d| {
            ks.iter().find(|x| x.at == d.at).map(|k| StochasticsEntry {
                at: k.at,
                k: k.value,
                d: d.value,
            })
        })
        .collect())
}

/// Returns Slow Stochastics for given Candlestick list
pub fn slow_stochastics(
    entries: &[Candlestick],
    duration_k: usize,
    duration_d: usize,
    duration_sd: usize,
) -> Result<Vec<SlowStochasticsEntry>, Box<dyn std::error::Error>> {
    if duration_k == 0 || duration_d == 0 || duration_sd == 0 || entries.len() < duration_k {
        return Ok(vec![]);
    }

    Candlestick::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());
    let ks = calc_k(&sorted, duration_k);
    let ds = sma(&ks, duration_d)?;
    let sds = sma(&ds, duration_sd)?;
    Ok(sds
        .iter()
        .map(|sd| SlowStochasticsEntry {
            at: sd.at,
            k: 0.0,
            d: 0.0,
            sd: sd.value,
        })
        .filter_map(|x| ds.iter().find(|d| d.at == x.at).map(|d| x.with_d(d.value)))
        .filter_map(|x| ks.iter().find(|k| k.at == x.at).map(|k| x.with_k(k.value)))
        .collect())
}

fn calc_k(entries: &[Candlestick], duration: usize) -> Vec<IndexEntry> {
    (0..entries.len() - duration + 1)
        .map(|i| {
            let xs = entries.iter().skip(i).take(duration);
            let lowest = xs.clone().map(|x| x.low).reduce(|z, x| z.min(x)).unwrap();
            let n = xs.clone().map(|x| x.high).reduce(|z, x| z.max(x)).unwrap() - lowest;
            let last = xs.last().unwrap();
            let k = if n == 0.0 {
                0.0
            } else {
                ((last.close - lowest) / n) * 100.0
            };
            IndexEntry {
                at: last.at,
                value: k,
            }
        })
        .collect()
}
