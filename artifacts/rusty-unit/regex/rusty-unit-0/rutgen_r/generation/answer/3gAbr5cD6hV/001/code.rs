// Answer 0

#[derive(Debug, PartialEq)]
struct ClassUnicodeRange {
    start: char,
    end: char,
}

#[derive(Debug, PartialEq)]
struct IntervalSet {
    ranges: Vec<ClassUnicodeRange>,
}

impl IntervalSet {
    fn new<I>(ranges: I) -> Self
    where
        I: IntoIterator<Item = ClassUnicodeRange>,
    {
        IntervalSet {
            ranges: ranges.into_iter().collect(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct ClassUnicode {
    set: IntervalSet,
}

fn new<I>(ranges: I) -> ClassUnicode
where
    I: IntoIterator<Item = ClassUnicodeRange>,
{
    ClassUnicode { set: IntervalSet::new(ranges) }
}

#[test]
fn test_new_empty_ranges() {
    let ranges: Vec<ClassUnicodeRange> = vec![];
    let result = new(ranges);
    assert_eq!(result, ClassUnicode { set: IntervalSet { ranges: vec![] } });
}

#[test]
fn test_new_single_range() {
    let ranges = vec![ClassUnicodeRange { start: 'a', end: 'z' }];
    let result = new(ranges);
    assert_eq!(result, ClassUnicode { set: IntervalSet { ranges: vec![ClassUnicodeRange { start: 'a', end: 'z' }] } });
}

#[test]
fn test_new_multiple_ranges() {
    let ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
        ClassUnicodeRange { start: 'x', end: 'z' },
    ];
    let result = new(ranges);
    assert_eq!(result, ClassUnicode { set: IntervalSet { ranges: vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
        ClassUnicodeRange { start: 'x', end: 'z' },
    ]}});
}

#[test]
fn test_new_overlapping_ranges() {
    let ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'f' },
        ClassUnicodeRange { start: 'e', end: 'j' },
    ];
    let result = new(ranges);
    assert_eq!(result, ClassUnicode { set: IntervalSet { ranges: vec![
        ClassUnicodeRange { start: 'a', end: 'f' },
        ClassUnicodeRange { start: 'e', end: 'j' },
    ]}});
}

#[test]
fn test_new_same_ranges() {
    let ranges = vec![
        ClassUnicodeRange { start: 'm', end: 'o' },
        ClassUnicodeRange { start: 'm', end: 'o' },
    ];
    let result = new(ranges);
    assert_eq!(result, ClassUnicode { set: IntervalSet { ranges: vec![
        ClassUnicodeRange { start: 'm', end: 'o' },
        ClassUnicodeRange { start: 'm', end: 'o' },
    ]}});
}

#[test]
fn test_new_reverse_order_ranges() {
    let ranges = vec![
        ClassUnicodeRange { start: 'z', end: 'a' },
        ClassUnicodeRange { start: 'b', end: 'd' },
    ];
    let result = new(ranges);
    assert_eq!(result, ClassUnicode { set: IntervalSet { ranges: vec![
        ClassUnicodeRange { start: 'z', end: 'a' },
        ClassUnicodeRange { start: 'b', end: 'd' },
    ]}});
}

