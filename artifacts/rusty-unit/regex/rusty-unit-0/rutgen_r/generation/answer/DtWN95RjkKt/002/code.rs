// Answer 0

#[derive(Debug)]
struct RepetitionRange {
    kind: RangeKind,
}

#[derive(Debug)]
enum RangeKind {
    Exactly(usize),
    AtLeast(usize),
    Bounded(usize, usize),
}

#[derive(Debug)]
enum RepetitionKind {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    Range(RepetitionRange),
}

#[derive(Debug)]
struct TestStruct {
    kind: RepetitionKind,
}

impl TestStruct {
    pub fn is_match_empty(&self) -> bool {
        match &self.kind {
            RepetitionKind::ZeroOrOne => true,
            RepetitionKind::ZeroOrMore => true,
            RepetitionKind::OneOrMore => false,
            RepetitionKind::Range(RepetitionRange { kind: RangeKind::Exactly(m) }) => *m == 0,
            RepetitionKind::Range(RepetitionRange { kind: RangeKind::AtLeast(m) }) => *m == 0,
            RepetitionKind::Range(RepetitionRange { kind: RangeKind::Bounded(m, _) }) => *m == 0,
        }
    }
}

#[test]
fn test_zero_or_one() {
    let test = TestStruct { kind: RepetitionKind::ZeroOrOne };
    assert_eq!(test.is_match_empty(), true);
}

#[test]
fn test_zero_or_more() {
    let test = TestStruct { kind: RepetitionKind::ZeroOrMore };
    assert_eq!(test.is_match_empty(), true);
}

#[test]
fn test_one_or_more() {
    let test = TestStruct { kind: RepetitionKind::OneOrMore };
    assert_eq!(test.is_match_empty(), false);
}

#[test]
fn test_exactly_zero() {
    let test = TestStruct { kind: RepetitionKind::Range(RepetitionRange { kind: RangeKind::Exactly(0) }) };
    assert_eq!(test.is_match_empty(), true);
}

#[test]
fn test_exactly_non_zero() {
    let test = TestStruct { kind: RepetitionKind::Range(RepetitionRange { kind: RangeKind::Exactly(1) }) };
    assert_eq!(test.is_match_empty(), false);
}

#[test]
fn test_at_least_zero() {
    let test = TestStruct { kind: RepetitionKind::Range(RepetitionRange { kind: RangeKind::AtLeast(0) }) };
    assert_eq!(test.is_match_empty(), true);
}

#[test]
fn test_at_least_non_zero() {
    let test = TestStruct { kind: RepetitionKind::Range(RepetitionRange { kind: RangeKind::AtLeast(1) }) };
    assert_eq!(test.is_match_empty(), false);
}

#[test]
fn test_bounded_zero() {
    let test = TestStruct { kind: RepetitionKind::Range(RepetitionRange { kind: RangeKind::Bounded(0, 1) }) };
    assert_eq!(test.is_match_empty(), true);
}

#[test]
fn test_bounded_non_zero() {
    let test = TestStruct { kind: RepetitionKind::Range(RepetitionRange { kind: RangeKind::Bounded(1, 2) }) };
    assert_eq!(test.is_match_empty(), false);
}

