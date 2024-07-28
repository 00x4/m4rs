//! Simple index data object

use std::fmt::Display;

use crate::Error;

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
    pub fn new(at: u64, value: f64) -> IndexEntry {
        IndexEntry { at, value }
    }

    /// Converts from IndexEntryLike to IndexEntry
    pub fn from<T: IndexEntryLike>(that: &T) -> IndexEntry {
        IndexEntry {
            at: that.get_at(),
            value: that.get_value(),
        }
    }

    pub(crate) fn validate_field(
        at: u64,
        v: f64,
        field: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if v.is_nan() {
            return Err(Box::new(Error::ContainsNaN {
                at,
                field: field.to_string(),
            }));
        }
        if v.is_infinite() {
            return Err(Box::new(Error::ContainsInfinite {
                at,
                field: field.to_string(),
            }));
        }
        Ok(())
    }

    pub(crate) fn validate_list<T: IndexEntryLike>(
        xs: &[T],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for x in xs {
            Self::validate_field(x.get_at(), x.get_value(), "value")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::f64::{INFINITY, NAN};

    use super::*;
    use crate::Error;

    #[test]
    fn test_validate_list() {
        // valid list
        let res = IndexEntry::validate_list(&vec![
            IndexEntry::new(1719400001, 100.0),
            IndexEntry::new(1719400002, 110.0),
            IndexEntry::new(1719400003, 130.0),
            IndexEntry::new(1719400004, 120.0),
            IndexEntry::new(1719400005, 90.0),
        ]);
        assert!(res.is_ok());

        // invalid: contains NAN
        let res = IndexEntry::validate_list(&vec![
            IndexEntry::new(1719400001, 100.0),
            IndexEntry::new(1719400002, 110.0),
            IndexEntry::new(1719400003, NAN),
            IndexEntry::new(1719400004, 120.0),
            IndexEntry::new(1719400005, 90.0),
        ]);
        assert!(res.is_err());
        let res = res.err().unwrap();
        let e = res.downcast_ref::<Error>();
        assert!(matches!(
            e,
            Some(Error::ContainsNaN {
                at: 1719400003,
                field: _
            })
        ));

        // invalid: contains INFINITY
        let res = IndexEntry::validate_list(&vec![
            IndexEntry::new(1719400001, 100.0),
            IndexEntry::new(1719400002, 110.0),
            IndexEntry::new(1719400003, 130.0),
            IndexEntry::new(1719400004, INFINITY),
            IndexEntry::new(1719400005, 90.0),
        ]);
        assert!(res.is_err());
        let res = res.err().unwrap();
        let e = res.downcast_ref::<Error>();
        assert!(matches!(
            e,
            Some(Error::ContainsInfinite {
                at: 1719400004,
                field: _
            })
        ));
    }
}
