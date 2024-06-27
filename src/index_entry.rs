use std::fmt::Display;

/// Abstract type of IndexEntry
pub trait IndexEntryLike: Clone {
    fn get_at(&self) -> u64;
    fn get_value(&self) -> f64;
}

/// Simple index entry
#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub at: u64,
    pub value: f64,
}

impl Display for IndexEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndexEntry(at={} value={})", self.at, self.value)
    }
}

impl IndexEntryLike for IndexEntry {
    fn get_at(&self) -> u64 {
        self.at
    }

    fn get_value(&self) -> f64 {
        self.value
    }
}

impl IndexEntry {
    /// Converts from IndexEntryLike to IndexEntry
    pub fn from<T: IndexEntryLike>(that: &T) -> IndexEntry {
        IndexEntry {
            at: that.get_at(),
            value: that.get_value(),
        }
    }
}
