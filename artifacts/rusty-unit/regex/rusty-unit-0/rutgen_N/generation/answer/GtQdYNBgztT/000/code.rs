// Answer 0

#[derive(Debug)]
struct ClassBytesRange {
    start: u8,
    end: u8,
}

impl ClassBytesRange {
    fn new(start: u8, end: u8) -> Self {
        ClassBytesRange { start, end }
    }

    fn is_intersection_empty(&self, other: &ClassBytesRange) -> bool {
        self.start > other.end || self.end < other.start
    }
}

struct MyStruct {
    start: u8,
    end: u8,
}

impl MyStruct {
    fn case_fold_simple(&self, ranges: &mut Vec<ClassBytesRange>) {
        if !ClassBytesRange::new(b'a', b'z').is_intersection_empty(&ClassBytesRange::new(self.start, self.end)) {
            let lower = self.start.max(b'a');
            let upper = self.end.min(b'z');
            ranges.push(ClassBytesRange::new(lower - 32, upper - 32));
        }
        if !ClassBytesRange::new(b'A', b'Z').is_intersection_empty(&ClassBytesRange::new(self.start, self.end)) {
            let lower = self.start.max(b'A');
            let upper = self.end.min(b'Z');
            ranges.push(ClassBytesRange::new(lower + 32, upper + 32));
        }
    }
}

#[test]
fn test_case_fold_simple_no_overlap() {
    let my_struct = MyStruct { start: b'0', end: b'9' };
    let mut ranges = Vec::new();
    my_struct.case_fold_simple(&mut ranges);
    assert!(ranges.is_empty());
}

#[test]
fn test_case_fold_simple_lower_case_overlap() {
    let my_struct = MyStruct { start: b'a', end: b'c' };
    let mut ranges = Vec::new();
    my_struct.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassBytesRange::new(b'A', b'C'));
}

#[test]
fn test_case_fold_simple_upper_case_overlap() {
    let my_struct = MyStruct { start: b'A', end: b'C' };
    let mut ranges = Vec::new();
    my_struct.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassBytesRange::new(b'a', b'c'));
}

#[test]
fn test_case_fold_simple_complete_overlap() {
    let my_struct = MyStruct { start: b'a', end: b'z' };
    let mut ranges = Vec::new();
    my_struct.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassBytesRange::new(b'A', b'Z'));
}

#[test]
fn test_case_fold_simple_complete_upper_case() {
    let my_struct = MyStruct { start: b'A', end: b'Z' };
    let mut ranges = Vec::new();
    my_struct.case_fold_simple(&mut ranges);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassBytesRange::new(b'a', b'z'));
}

