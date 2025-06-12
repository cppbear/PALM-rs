// Answer 0

#[derive(Debug)]
struct ClassUnicodeRange {
    start: char,
    end: char,
}

struct UnicodeRangeSet {
    set: Vec<ClassUnicodeRange>,
}

impl UnicodeRangeSet {
    pub fn new() -> Self {
        UnicodeRangeSet { set: Vec::new() }
    }

    pub fn push(&mut self, range: ClassUnicodeRange) {
        self.set.push(range);
    }
}

#[test]
fn test_push_single_range() {
    let mut unicode_range_set = UnicodeRangeSet::new();
    let range = ClassUnicodeRange { start: 'a', end: 'z' };
    unicode_range_set.push(range);
    assert_eq!(unicode_range_set.set.len(), 1);
    assert_eq!(unicode_range_set.set[0].start, 'a');
    assert_eq!(unicode_range_set.set[0].end, 'z');
}

#[test]
fn test_push_multiple_ranges() {
    let mut unicode_range_set = UnicodeRangeSet::new();
    let range1 = ClassUnicodeRange { start: 'a', end: 'z' };
    let range2 = ClassUnicodeRange { start: 'A', end: 'Z' };
    unicode_range_set.push(range1);
    unicode_range_set.push(range2);
    assert_eq!(unicode_range_set.set.len(), 2);
    assert_eq!(unicode_range_set.set[1].start, 'A');
    assert_eq!(unicode_range_set.set[1].end, 'Z');
}

#[test]
fn test_push_empty_range() {
    let mut unicode_range_set = UnicodeRangeSet::new();
    let range = ClassUnicodeRange { start: '1', end: '1' };
    unicode_range_set.push(range);
    assert_eq!(unicode_range_set.set.len(), 1);
    assert_eq!(unicode_range_set.set[0].start, '1');
    assert_eq!(unicode_range_set.set[0].end, '1');
}

