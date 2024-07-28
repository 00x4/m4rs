//! Ichimoku Kinko Hyo
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
//! // Get Ichimoku calculation result with default common parameters
//! let result = m4rs::ichimoku_default(&candlesticks);
//! ```

use super::Candlestick;
use super::IndexEntry;
use super::IndexEntryLike;

#[derive(Debug)]
pub struct IchimokuEntry {
    pub at: u64,
    pub conversion_line: Option<f64>,
    pub base_line: Option<f64>,
    pub leading_span_a: Option<f64>,
    pub leading_span_b: Option<f64>,
    pub lagging_span: Option<f64>,
}

#[derive(Debug)]
pub struct IchimokuData {
    pub conversion_line: Vec<IndexEntry>,
    pub base_line: Vec<IndexEntry>,
    pub leading_span_a: Vec<IndexEntry>,
    pub leading_span_b: Vec<IndexEntry>,
    pub lagging_span: Vec<IndexEntry>,
}

impl IchimokuData {
    pub fn get(&self, at: u64) -> Option<IchimokuEntry> {
        let conversion_line = self
            .conversion_line
            .iter()
            .find(|x| x.at == at)
            .map(|x| x.value);
        let base_line = self.base_line.iter().find(|x| x.at == at).map(|x| x.value);
        let leading_span_a = self
            .leading_span_a
            .iter()
            .find(|x| x.at == at)
            .map(|x| x.value);
        let leading_span_b = self
            .leading_span_b
            .iter()
            .find(|x| x.at == at)
            .map(|x| x.value);
        let lagging_span = self
            .lagging_span
            .iter()
            .find(|x| x.at == at)
            .map(|x| x.value);
        if [
            conversion_line,
            base_line,
            leading_span_a,
            leading_span_b,
            lagging_span,
        ]
        .iter()
        .all(|x| x.is_none())
        {
            return None;
        }
        Some(IchimokuEntry {
            at,
            conversion_line,
            base_line,
            leading_span_a,
            leading_span_b,
            lagging_span,
        })
    }
}

/// Returns Ichimoku Kinkohyo for given Candlestick list with default parameters
pub fn ichimoku_default(
    entries: &[Candlestick],
) -> Result<IchimokuData, Box<dyn std::error::Error>> {
    ichimoku(entries, 9, 26, 52, 26)
}

/// Returns Ichimoku Kinkohyo for given Candlestick list with custom parameters
pub fn ichimoku(
    entries: &[Candlestick],
    conversion_line_len: usize,
    base_line_len: usize,
    leading_span_b_len: usize,
    lagging_span: usize,
) -> Result<IchimokuData, Box<dyn std::error::Error>> {
    Candlestick::validate_list(entries)?;

    let base_line = calc_base_and_conversion_line(entries, base_line_len);
    let conversion_line = calc_base_and_conversion_line(entries, conversion_line_len);
    Ok(IchimokuData {
        conversion_line: conversion_line.clone(),
        base_line: base_line.clone(),
        leading_span_a: calc_leading_span_a(&base_line, &conversion_line, lagging_span),
        leading_span_b: calc_leading_span_b(entries, leading_span_b_len, lagging_span),
        lagging_span: calc_lagging_span(entries, lagging_span),
    })
}

fn calc_base_and_conversion_line(entries: &[Candlestick], line_len: usize) -> Vec<IndexEntry> {
    if line_len == 0 {
        return vec![];
    }
    let mut ret = Vec::<IndexEntry>::new();
    for i in 0..entries.len() {
        if entries.len() < i + line_len {
            break;
        }
        let xs = &entries[i..i + line_len];
        let highest = xs.iter().fold(
            -1.0,
            |z, x| if z == -1.0 || z < x.high { x.high } else { z },
        );
        let lowest = xs
            .iter()
            .fold(-1.0, |z, x| if z == -1.0 || z > x.low { x.low } else { z });
        ret.push(IndexEntry {
            at: xs.last().unwrap().at,
            value: (highest + lowest) / 2.0,
        });
    }
    ret
}

fn calc_leading_span_a(
    base_line: &[IndexEntry],
    conversion_line: &[IndexEntry],
    span: usize,
) -> Vec<IndexEntry> {
    let entries: Vec<IndexEntry> = base_line
        .iter()
        .filter_map(|b| {
            conversion_line
                .iter()
                .find(|c| c.at == b.at)
                .map(|c| (b, c))
        })
        .map(|(b, c)| IndexEntry {
            at: b.at,
            value: (b.value + c.value) / 2.0,
        })
        .collect();
    apply_lag(&entries, span, false)
}

fn calc_leading_span_b(entries: &[Candlestick], line_len: usize, span: usize) -> Vec<IndexEntry> {
    apply_lag(
        &calc_base_and_conversion_line(entries, line_len),
        span,
        false,
    )
}

fn calc_lagging_span(entries: &[Candlestick], span: usize) -> Vec<IndexEntry> {
    apply_lag(entries, span, true)
}

fn apply_lag(entries: &[impl IndexEntryLike], span: usize, backward: bool) -> Vec<IndexEntry> {
    if entries.len() < 2 || span == 0 {
        return entries.iter().map(|x| IndexEntry::from(x)).collect();
    }
    let len = entries.len();
    let last = entries.last().unwrap();
    let prev = &entries[len - 2];
    let duration = (last.get_at() - prev.get_at()) * (span - 1) as u64;
    entries
        .iter()
        .enumerate()
        .map(|(i, x)| IndexEntry {
            at: {
                let pos = i as u32 + span as u32;
                if 0 == pos && pos < len as u32 {
                    entries[pos as usize].get_at()
                } else if backward {
                    x.get_at() - duration
                } else {
                    x.get_at() + duration
                }
            },
            value: x.get_value(),
        })
        .collect()
}
