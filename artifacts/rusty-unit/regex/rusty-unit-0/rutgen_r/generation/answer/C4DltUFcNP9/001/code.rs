// Answer 0

#[derive(Debug)]
struct Range {
    start: char,
    end: char,
}

impl Range {
    fn new(start: char, end: char) -> Self {
        Self { start, end }
    }
    
    fn case_fold_simple(&self, ranges: &mut Vec<Range>) {
        let start_folded = self.start.to_lowercase().next().unwrap();
        let end_folded = self.end.to_uppercase().next().unwrap();
        
        ranges.push(Range::new(start_folded, end_folded));
    }
}

struct IntervalSet {
    ranges: Vec<Range>,
}

impl IntervalSet {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }
    
    pub fn case_fold_simple(&mut self) {
        let len = self.ranges.len();
        for i in 0..len {
            let range = &self.ranges[i];
            range.case_fold_simple(&mut self.ranges);
        }
        self.canonicalize();
    }
    
    fn canonicalize(&mut self) {
        // Suppose the canonicalize function simply removes duplicates and
        // sorts the ranges after case folding.
        self.ranges.sort_by(|a, b| a.start.cmp(&b.start));
        self.ranges.dedup();
    }
}

#[test]
fn test_case_fold_simple_with_single_range() {
    let mut interval_set = IntervalSet::new();
    interval_set.ranges.push(Range::new('a', 'z'));
    interval_set.case_fold_simple();
    assert_eq!(interval_set.ranges.len(), 2);
    assert_eq!(interval_set.ranges[0], Range::new('a', 'z'));
    assert_eq!(interval_set.ranges[1], Range::new('A', 'Z'));
}

#[test]
fn test_case_fold_simple_with_empty_ranges() {
    let mut interval_set = IntervalSet::new();
    interval_set.case_fold_simple();
    assert_eq!(interval_set.ranges.len(), 0);
}

#[test]
fn test_case_fold_simple_with_non_contiguous_ranges() {
    let mut interval_set = IntervalSet::new();
    interval_set.ranges.push(Range::new('c', 'f'));
    interval_set.ranges.push(Range::new('n', 'p'));
    interval_set.case_fold_simple();
    assert_eq!(interval_set.ranges.len(), 4);
    assert_eq!(interval_set.ranges[0], Range::new('c', 'f'));
    assert_eq!(interval_set.ranges[1], Range::new('C', 'F'));
    assert_eq!(interval_set.ranges[2], Range::new('n', 'p'));
    assert_eq!(interval_set.ranges[3], Range::new('N', 'P'));
}

#[test]
#[should_panic]
fn test_case_fold_simple_out_of_bounds() {
    let mut interval_set = IntervalSet::new();
    // Intentionally access an out of bounds index
    interval_set.ranges.push(Range::new('x', 'y'));
    interval_set.case_fold_simple(); // Ensure panics do not occur here
    let _ = interval_set.ranges[2]; // This should panic
}

