//! DMI (Directional Movement Index)
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
//! // Get DMI calculation result
//! let result = m4rs::dmi(&candlesticks, 14);
//! ```

use std::fmt::Display;

use crate::{ema, Candlestick, Error, IndexEntry, IndexEntryLike};

#[derive(Clone, Debug)]
pub struct DmiEntry {
    pub at: u64,
    pub plus_di: f64,
    pub minus_di: f64,
    pub dx: f64,
    pub adx: f64,
}

impl DmiEntry {
    fn with_adx(&self, adx: f64) -> DmiEntry {
        DmiEntry {
            at: self.at,
            plus_di: self.plus_di,
            minus_di: self.minus_di,
            dx: self.dx,
            adx,
        }
    }
}

impl Display for DmiEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Dmi(at={} +di={} -di={} dx={} adx={})",
            self.at, self.plus_di, self.minus_di, self.dx, self.adx
        )
    }
}

struct Calc {
    at: u64,
    plus_dm: f64,
    minus_dm: f64,
    tr: f64,
}

impl Calc {
    fn plus_dm(&self) -> IndexEntry {
        IndexEntry {
            at: self.at,
            value: self.plus_dm,
        }
    }

    fn minus_dm(&self) -> IndexEntry {
        IndexEntry {
            at: self.at,
            value: self.minus_dm,
        }
    }

    fn tr(&self) -> IndexEntry {
        IndexEntry {
            at: self.at,
            value: self.tr,
        }
    }
}

/// Returns DMI/ADX for given IndexEntry list
pub fn dmi(entries: &[Candlestick], duration: usize) -> Result<Vec<DmiEntry>, Error> {
    if duration == 0 || entries.len() < duration {
        return Ok(vec![]);
    }
    Candlestick::validate_list(entries)?;

    let mut sorted = entries.to_owned();
    sorted.sort_by(|a, b| a.at.cmp(&b.at));

    let calcs = calc_dm(&sorted);
    let plus_dm_ma = wilder_ma(
        &calcs
            .iter()
            .map(|x| x.plus_dm())
            .collect::<Vec<IndexEntry>>(),
        duration,
    )?;
    let minus_dm_ma = wilder_ma(
        &calcs
            .iter()
            .map(|x| x.minus_dm())
            .collect::<Vec<IndexEntry>>(),
        duration,
    )?;
    let tr_ma = wilder_ma(
        &calcs.iter().map(|x| x.tr()).collect::<Vec<IndexEntry>>(),
        duration,
    )?;
    let dmis = tr_ma.iter().filter_map(|tr| {
        match (
            plus_dm_ma.iter().find(|x| x.at == tr.at),
            minus_dm_ma.iter().find(|x| x.at == tr.at),
        ) {
            (None, _) | (_, None) => None,
            (Some(plus_dm), Some(minus_dm)) => {
                let plus_di = plus_dm.value / tr.value;
                let minus_di = minus_dm.value / tr.value;
                Some(DmiEntry {
                    at: tr.at,
                    plus_di,
                    minus_di,
                    dx: (plus_di - minus_di).abs() / (plus_di + minus_di),
                    adx: 0.0,
                })
            }
        }
    });
    let adxs = wilder_ma(
        &dmis
            .clone()
            .map(|x| IndexEntry {
                at: x.at,
                value: x.dx,
            })
            .collect::<Vec<IndexEntry>>(),
        duration,
    )?;
    Ok(dmis
        .filter_map(|dmi| {
            adxs.iter()
                .find(|x| x.at == dmi.at)
                .map(|adx| dmi.with_adx(adx.value))
        })
        .collect())
}

fn calc_dm(entries: &[Candlestick]) -> Vec<Calc> {
    let mut prev_entries = entries.to_vec();
    prev_entries.pop();
    let cur_entries = entries.iter().skip(1);
    prev_entries
        .iter()
        .zip(cur_entries)
        .map(|(prev, cur)| {
            let plus_dm_tmp = (cur.high - prev.high).max(0.0);
            let minus_dm_tmp = (prev.low - cur.low).max(0.0);
            let plus_dm = if plus_dm_tmp < minus_dm_tmp {
                0.0
            } else {
                plus_dm_tmp
            };
            let minus_dm = if plus_dm_tmp > minus_dm_tmp {
                0.0
            } else {
                minus_dm_tmp
            };
            Calc {
                at: cur.at,
                plus_dm,
                minus_dm,
                tr: [
                    cur.high - cur.low,
                    (cur.high - prev.close).abs(),
                    (prev.close - cur.low).abs(),
                ]
                .iter()
                .fold(0.0, |z, x| z.max(*x)),
            }
        })
        .collect()
}

fn wilder_ma(entries: &[impl IndexEntryLike], duration: usize) -> Result<Vec<IndexEntry>, Error> {
    ema(entries, duration * 2 - 1)
}
