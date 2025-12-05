// Contains a range [start, end)
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Range {
        Self { start, end }
    }

    pub fn length(&self) -> u64 {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    // Split into two non-overlapping ranges. If length >= self.length, the
    // first range will be self and the second range will be empty.
    pub fn split_front(&self, length: u64) -> (Range, Range) {
        let a = Range {
            start: self.start,
            end: (self.start + length).min(self.end),
        };
        let b = Range {
            start: (self.start + length).min(self.end),
            end: self.end,
        };

        (a, b)
    }

    // Split into two non-overlapping ranges. If length >= self.length, the
    // first range will be empty and the second range will be self.
    pub fn split_back(&self, length: u64) -> (Range, Range) {
        let a = Range {
            start: self.start,
            end: self.end.saturating_sub(length).max(self.start),
        };
        let b = Range {
            start: self.end.saturating_sub(length).max(self.start),
            end: self.end,
        };

        (a, b)
    }

    pub fn iter(&self) -> impl Iterator<Item = u64> {
        self.start..self.end
    }

    pub fn sum(&self) -> u64 {
        self.iter().sum()
    }

    pub fn contains(&self, n: u64) -> bool {
        n >= self.start && n < self.end
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        !(self.end <= other.start || other.end <= self.start)
    }

    pub fn merge(&self, other: &Range) -> Option<Range> {
        if !self.overlaps(other) {
            return None;
        }

        Some(Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        })
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {})", self.start, self.end)
    }
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RangeSet {
    ranges: Vec<Range>,
}

impl RangeSet {
    pub fn len(&self) -> u64 {
        self.ranges.iter().map(|r| r.length()).sum()
    }

    pub fn contains(&self, n: u64) -> bool {
        match self.ranges.binary_search_by_key(&n, |r| r.end) {
            Ok(_) => false,
            Err(i) => self.ranges.get(i).map(|r| r.contains(n)).unwrap_or(false),
        }
    }
}

impl From<Vec<Range>> for RangeSet {
    fn from(mut ranges: Vec<Range>) -> Self {
        ranges.sort_unstable_by_key(|r| r.start);

        let mut i = 0;
        let mut j = 1;

        while j < ranges.len() {
            if let Some(merged) = ranges[i].merge(&ranges[j]) {
                ranges[i] = merged;
            } else {
                i += 1;
                ranges[i] = ranges[j];
            }

            j += 1;
        }

        ranges.truncate(i + 1);

        Self { ranges }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_is_empty_test() {
        assert!(Range::new(0, 0).is_empty());
        // non-canonical range
        assert!(Range::new(20, 1).is_empty());

        // not zero
        assert!(!Range::new(0, 1).is_empty());
    }

    #[test]
    fn split_front_test() {
        let r = Range::new(0, 10);
        assert_eq!(r.split_front(4), (Range::new(0, 4), Range::new(4, 10)));
        assert_eq!(r.split_front(10), (r, Range::new(10, 10)));
        assert_eq!(r.split_front(11), (r, Range::new(10, 10)));
    }

    #[test]
    fn split_back_test() {
        let r = Range::new(0, 10);
        assert_eq!(r.split_back(4), (Range::new(0, 6), Range::new(6, 10)));
        assert_eq!(r.split_back(10), (Range::new(0, 0), r));
        assert_eq!(r.split_back(11), (Range::new(0, 0), r));
    }
}
