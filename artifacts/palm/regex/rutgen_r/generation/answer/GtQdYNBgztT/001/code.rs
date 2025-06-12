// Answer 0

#[test]
fn test_case_fold_simple_no_intersection() {
    struct TestRange {
        start: u8,
        end: u8,
    }

    impl TestRange {
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

    let range = TestRange { start: 0, end: 127 };
    let mut ranges: Vec<ClassBytesRange> = Vec::new();
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 0);
}

#[test]
fn test_case_fold_simple_with_non_ascii() {
    struct TestRange {
        start: u8,
        end: u8,
    }

    impl TestRange {
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

    let range = TestRange { start: b'[' as u8, end: b'`' as u8 };
    let mut ranges: Vec<ClassBytesRange> = Vec::new();
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 0);
}

#[test]
fn test_case_fold_simple_edge_case() {
    struct TestRange {
        start: u8,
        end: u8,
    }

    impl TestRange {
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

    let range = TestRange { start: b'`' as u8, end: b'a' as u8 }; // overlapping a single case
    let mut ranges: Vec<ClassBytesRange> = Vec::new();
    range.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 0);  // still no valid ranges added
}

