// Answer 0

#[derive(Debug)]
struct ClassUnicodeRange {
    start: char,
    end: char,
}

struct UnicodeClass {
    set: UnicodeSet,
}

impl UnicodeClass {
    pub fn ranges(&self) -> &[ClassUnicodeRange] {
        self.set.intervals()
    }
}

struct UnicodeSet {
    intervals: Vec<ClassUnicodeRange>,
}

impl UnicodeSet {
    pub fn new(intervals: Vec<ClassUnicodeRange>) -> Self {
        UnicodeSet { intervals }
    }

    pub fn intervals(&self) -> &[ClassUnicodeRange] {
        &self.intervals
    }
}

#[test]
fn test_ranges_empty() {
    let unicode_set = UnicodeSet::new(vec![]);
    let unicode_class = UnicodeClass { set: unicode_set };
    assert_eq!(unicode_class.ranges().len(), 0);
}

#[test]
fn test_ranges_single() {
    let unicode_set = UnicodeSet::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]);
    let unicode_class = UnicodeClass { set: unicode_set };
    assert_eq!(unicode_class.ranges().len(), 1);
    assert_eq!(unicode_class.ranges()[0], ClassUnicodeRange { start: 'a', end: 'z' });
}

#[test]
fn test_ranges_multiple() {
    let unicode_set = UnicodeSet::new(vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
        ClassUnicodeRange { start: 'A', end: 'Z' },
    ]);
    let unicode_class = UnicodeClass { set: unicode_set };
    assert_eq!(unicode_class.ranges().len(), 2);
    assert_eq!(unicode_class.ranges()[0], ClassUnicodeRange { start: 'a', end: 'z' });
    assert_eq!(unicode_class.ranges()[1], ClassUnicodeRange { start: 'A', end: 'Z' });
}

