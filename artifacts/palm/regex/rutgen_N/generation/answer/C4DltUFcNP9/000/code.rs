// Answer 0

#[derive(Debug)]
struct Interval {
    start: char,
    end: char,
}

impl Interval {
    fn new(start: char, end: char) -> Self {
        Interval { start, end }
    }

    fn case_fold_simple(&self, ranges: &mut Vec<Interval>) {
        // Simplified case folding for lowercase a-z to uppercase A-Z
        if self.start.is_ascii_lowercase() && self.end.is_ascii_lowercase() {
            let new_start = self.start.to_ascii_uppercase();
            let new_end = self.end.to_ascii_uppercase();
            ranges.push(Interval::new(new_start, new_end));
        }
        ranges.push(*self);
    }
}

#[derive(Debug)]
struct IntervalSet {
    ranges: Vec<Interval>,
}

impl IntervalSet {
    fn new() -> Self {
        IntervalSet { ranges: Vec::new() }
    }

    fn case_fold_simple(&mut self) {
        let len = self.ranges.len();
        for i in 0..len {
            let range = &self.ranges[i];
            range.case_fold_simple(&mut self.ranges);
        }
        self.canonicalize();
    }

    fn canonicalize(&mut self) {
        // Simplified canonicalization: merging overlapping ranges
        self.ranges.sort_by(|a, b| a.start.cmp(&b.start));
        self.ranges.dedup();
    }
}

#[test]
fn test_case_fold_simple_basic() {
    let mut interval_set = IntervalSet::new();
    interval_set.ranges.push(Interval::new('a', 'z'));
    interval_set.case_fold_simple();
    assert_eq!(interval_set.ranges.len(), 2);
    assert_eq!(interval_set.ranges[0], Interval::new('a', 'z'));
    assert_eq!(interval_set.ranges[1], Interval::new('A', 'Z'));
}

#[test]
fn test_case_fold_simple_partial() {
    let mut interval_set = IntervalSet::new();
    interval_set.ranges.push(Interval::new('a', 'c'));
    interval_set.ranges.push(Interval::new('d', 'd'));
    interval_set.case_fold_simple();
    assert_eq!(interval_set.ranges.len(), 4);
    assert_eq!(interval_set.ranges[0], Interval::new('a', 'c'));
    assert_eq!(interval_set.ranges[1], Interval::new('A', 'C'));
    assert_eq!(interval_set.ranges[2], Interval::new('d', 'd'));
    assert_eq!(interval_set.ranges[3], Interval::new('D', 'D'));
}

#[test]
fn test_case_fold_simple_no_lowercase() {
    let mut interval_set = IntervalSet::new();
    interval_set.ranges.push(Interval::new('A', 'Z'));
    interval_set.case_fold_simple();
    assert_eq!(interval_set.ranges.len(), 1);
    assert_eq!(interval_set.ranges[0], Interval::new('A', 'Z'));
}

#[test]
fn test_case_fold_simple_empty() {
    let mut interval_set = IntervalSet::new();
    interval_set.case_fold_simple();
    assert!(interval_set.ranges.is_empty());
}

