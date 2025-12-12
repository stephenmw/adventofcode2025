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

    pub fn iter(&self) -> impl Iterator<Item = u64> {
        self.start..self.end
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
