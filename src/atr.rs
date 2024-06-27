use super::rma;
use super::Candlestick;
use super::IndexEntry;

/// Returns ATR (Average True Range) for given Candlestick list
pub fn atr(entries: &[Candlestick], duration: usize) -> Vec<IndexEntry> {
    if duration == 0 || entries.len() < duration {
        return vec![];
    }
    let mut sorted = entries.to_owned();
    sorted.sort_by(|a, b| a.at.cmp(&b.at));
    let mut tr: Vec<IndexEntry> = vec![];
    for (i, x) in sorted.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let prev = sorted.get(i - 1).unwrap();
        let r1 = x.high - prev.close;
        let r2 = (x.low - prev.close).abs();
        let r3 = (x.high - x.low).abs();
        tr.push(IndexEntry {
            at: x.at,
            value: r1.max(r2).max(r3),
        });
    }
    rma(&tr, duration)
}
