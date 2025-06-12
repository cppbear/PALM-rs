// Answer 0

#[derive(Debug)]
struct ClassUnicodeRange {
    start: u32,
    end: u32,
}

struct Set {
    intervals: Vec<ClassUnicodeRange>,
}

impl Set {
    fn new(intervals: Vec<ClassUnicodeRange>) -> Self {
        Set { intervals }
    }

    fn intervals(&self) -> &[ClassUnicodeRange] {
        &self.intervals
    }
}

struct MyStruct {
    set: Set,
}

impl MyStruct {
    fn new(set: Set) -> Self {
        MyStruct { set }
    }

    pub fn ranges(&self) -> &[ClassUnicodeRange] {
        self.set.intervals()
    }
}

#[test]
fn test_ranges_empty() {
    let set = Set::new(vec![]);
    let my_struct = MyStruct::new(set);
    assert_eq!(my_struct.ranges().len(), 0);
}

#[test]
fn test_ranges_single_range() {
    let set = Set::new(vec![ClassUnicodeRange { start: 0, end: 10 }]);
    let my_struct = MyStruct::new(set);
    let ranges = my_struct.ranges();
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0].start, 0);
    assert_eq!(ranges[0].end, 10);
}

#[test]
fn test_ranges_multiple_ranges() {
    let set = Set::new(vec![
        ClassUnicodeRange { start: 0, end: 10 },
        ClassUnicodeRange { start: 20, end: 30 },
    ]);
    let my_struct = MyStruct::new(set);
    let ranges = my_struct.ranges();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0].start, 0);
    assert_eq!(ranges[0].end, 10);
    assert_eq!(ranges[1].start, 20);
    assert_eq!(ranges[1].end, 30);
}

