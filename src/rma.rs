use super::ema_with_alpha;
use super::IndexEntry;
use super::IndexEntryLike;

/// Returns RMA (Running Moving Average) for given IndexEntry list
pub fn rma(entries: &[impl IndexEntryLike], duration: usize) -> Vec<IndexEntry> {
    ema_with_alpha(entries, duration, 1.0 / (duration as f64))
}
