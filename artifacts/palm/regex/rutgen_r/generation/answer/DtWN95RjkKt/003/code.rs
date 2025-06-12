// Answer 0

#[derive(Debug)]
enum RepetitionKind {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    Range(RepetitionRange),
}

#[derive(Debug)]
enum RepetitionRange {
    Exactly(usize),
    AtLeast(usize),
    Bounded(usize, usize),
}

struct Hir {
    kind: RepetitionKind,
}

impl Hir {
    pub fn is_match_empty(&self) -> bool {
        match self.kind {
            RepetitionKind::ZeroOrOne => true,
            RepetitionKind::ZeroOrMore => true,
            RepetitionKind::OneOrMore => false,
            RepetitionKind::Range(RepetitionRange::Exactly(m)) => m == 0,
            RepetitionKind::Range(RepetitionRange::AtLeast(m)) => m == 0,
            RepetitionKind::Range(RepetitionRange::Bounded(m, _)) => m == 0,
        }
    }
}

#[test]
fn test_zero_or_one() {
    let hir = Hir {
        kind: RepetitionKind::ZeroOrOne,
    };
    assert!(hir.is_match_empty());
}

#[test]
fn test_zero_or_more() {
    let hir = Hir {
        kind: RepetitionKind::ZeroOrMore,
    };
    assert!(hir.is_match_empty());
}

#[test]
fn test_one_or_more() {
    let hir = Hir {
        kind: RepetitionKind::OneOrMore,
    };
    assert!(!hir.is_match_empty());
}

#[test]
fn test_range_exactly_zero() {
    let hir = Hir {
        kind: RepetitionKind::Range(RepetitionRange::Exactly(0)),
    };
    assert!(hir.is_match_empty());
}

#[test]
fn test_range_exactly_nonzero() {
    let hir = Hir {
        kind: RepetitionKind::Range(RepetitionRange::Exactly(1)),
    };
    assert!(!hir.is_match_empty());
}

#[test]
fn test_range_at_least_zero() {
    let hir = Hir {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(0)),
    };
    assert!(hir.is_match_empty());
}

#[test]
fn test_range_at_least_nonzero() {
    let hir = Hir {
        kind: RepetitionKind::Range(RepetitionRange::AtLeast(1)),
    };
    assert!(!hir.is_match_empty());
}

#[test]
fn test_range_bounded_zero() {
    let hir = Hir {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(0, 10)),
    };
    assert!(hir.is_match_empty());
}

#[test]
fn test_range_bounded_nonzero() {
    let hir = Hir {
        kind: RepetitionKind::Range(RepetitionRange::Bounded(1, 10)),
    };
    assert!(!hir.is_match_empty());
}

