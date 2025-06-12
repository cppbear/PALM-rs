// Answer 0

fn test_case_fold_simple_both_lower_and_upper() {
    struct BytesRange {
        start: u8,
        end: u8,
    }

    impl BytesRange {
        fn new(start: u8, end: u8) -> Self {
            Self { start, end }
        }

        fn case_fold_simple(&self, ranges: &mut Vec<ClassBytesRange>) {
            if !ClassBytesRange::new(b'a', b'z').is_intersection_empty(self) {
                let lower = cmp::max(self.start, b'a');
                let upper = cmp::min(self.end, b'z');
                ranges.push(ClassBytesRange::new(lower - 32, upper - 32));
            }
            if !ClassBytesRange::new(b'A', b'Z').is_intersection_empty(self) {
                let lower = cmp::max(self.start, b'A');
                let upper = cmp::min(self.end, b'Z');
                ranges.push(ClassBytesRange::new(lower + 32, upper + 32));
            }
        }
    }

    let mut ranges = Vec::new();
    let self_range = BytesRange::new(b'a', b'z'); // Covers lower case
    self_range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassBytesRange::new(b'A', b'Z'));

    ranges.clear();
    let self_range_upper = BytesRange::new(b'A', b'Z'); // Covers upper case
    self_range_upper.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassBytesRange::new(b'a', b'z'));
}

fn test_case_fold_simple_overlap_case() {
    struct BytesRange {
        start: u8,
        end: u8,
    }

    impl BytesRange {
        fn new(start: u8, end: u8) -> Self {
            Self { start, end }
        }

        fn case_fold_simple(&self, ranges: &mut Vec<ClassBytesRange>) {
            if !ClassBytesRange::new(b'a', b'z').is_intersection_empty(self) {
                let lower = cmp::max(self.start, b'a');
                let upper = cmp::min(self.end, b'z');
                ranges.push(ClassBytesRange::new(lower - 32, upper - 32));
            }
            if !ClassBytesRange::new(b'A', b'Z').is_intersection_empty(self) {
                let lower = cmp::max(self.start, b'A');
                let upper = cmp::min(self.end, b'Z');
                ranges.push(ClassBytesRange::new(lower + 32, upper + 32));
            }
        }
    }

    let mut ranges = Vec::new();
    let self_range_overlap = BytesRange::new(b'c', b'x'); // Overlaps both cases
    self_range_overlap.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], ClassBytesRange::new(b'A', b'B')); // For lower case
    assert_eq!(ranges[1], ClassBytesRange::new(b'a', b'w')); // For upper case
}

fn test_case_fold_simple_empty_non_overlap() {
    struct BytesRange {
        start: u8,
        end: u8,
    }

    impl BytesRange {
        fn new(start: u8, end: u8) -> Self {
            Self { start, end }
        }

        fn case_fold_simple(&self, ranges: &mut Vec<ClassBytesRange>) {
            if !ClassBytesRange::new(b'a', b'z').is_intersection_empty(self) {
                let lower = cmp::max(self.start, b'a');
                let upper = cmp::min(self.end, b'z');
                ranges.push(ClassBytesRange::new(lower - 32, upper - 32));
            }
            if !ClassBytesRange::new(b'A', b'Z').is_intersection_empty(self) {
                let lower = cmp::max(self.start, b'A');
                let upper = cmp::min(self.end, b'Z');
                ranges.push(ClassBytesRange::new(lower + 32, upper + 32));
            }
        }
    }

    let mut ranges = Vec::new();
    let self_range_empty = BytesRange::new(b'!', b'/'); // Non-overlapping with both
    self_range_empty.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 0);
}

