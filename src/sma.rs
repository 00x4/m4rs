use super::{IndexEntry, IndexEntryLike};

/// Returns SMA (Simple Moving Average) for given IndexEntry list
pub fn sma(entries: &[impl IndexEntryLike], duration: usize) -> Vec<IndexEntry> {
    if duration == 0 || entries.len() < duration {
        return vec![];
    }
    let mut sorted = entries.to_owned();
    sorted.sort_by_key(|x| x.get_at());
    (0..=(sorted.len() - duration))
        .map(|i| sorted.iter().skip(i).take(duration))
        .map(|xs| IndexEntry {
            at: xs.clone().last().unwrap().get_at(),
            value: xs.fold(0.0, |z, x| z + x.get_value()) / (duration as f64),
        })
        .collect()
}
