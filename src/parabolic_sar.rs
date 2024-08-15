//! Parabolic SAR (Stop And Reverse)
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
//! // Get Parabolic SAR calculation result
//! let result = m4rs::parabolic_sar(&candlesticks, 0.02, 0.02, 0.2);
//! ```

use crate::{Candlestick, Error, IndexEntry};

/// Returns Parabolic SAR for given Candlestick list
pub fn parabolic_sar(
    entries: &[Candlestick],
    af_init: f32,
    af_step: f32,
    af_max: f32,
) -> Result<Vec<IndexEntry>, Error> {
    let af_init = validate_arg(af_init, "af_init")?;
    let af_step = validate_arg(af_step, "af_step")?;
    let af_max = validate_arg(af_max, "af_max")?;

    if entries.is_empty() {
        return Ok(vec![]);
    }
    Candlestick::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by(|a, b| a.at.cmp(&b.at));

    let head = sorted.first().unwrap();
    let mut is_bullish = head.is_bullish();
    let mut af = af_init;
    let mut ep = if is_bullish { head.high } else { head.low };
    let mut sar = if is_bullish { head.low } else { head.high };

    let mut r: Vec<IndexEntry> = vec![];

    for x in sorted.iter().skip(1) {
        r.push(IndexEntry::new(x.at, sar));

        if is_bullish && sar > x.low {
            is_bullish = false;
            af = af_init;
            sar = ep;
            ep = x.low;
        } else if !is_bullish && sar < x.high {
            is_bullish = true;
            af = af_init;
            sar = ep;
            ep = x.high;
        } else {
            if is_bullish && ep < x.high {
                af += af_step;
                ep = x.high;
            } else if !is_bullish && ep > x.low {
                af += af_step;
                ep = x.low;
            }
            if af > af_max {
                af = af_max;
            }
        }

        sar = sar + af * (ep - sar);
    }

    Ok(r)
}

fn validate_arg(value: f32, field: &str) -> Result<f64, Error> {
    if value < 0.0 {
        return Err(Error::MustBePositiveF32 {
            value,
            field: field.to_string(),
        });
    }
    Ok(value as f64)
}
