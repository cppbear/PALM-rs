// Answer 0

#[test]
fn test_repetition_empty() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    enum RepetitionKind {
        ZeroOrOne,
        ZeroOrMore,
        OneOrMore,
        Range(RepetitionRange),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum RepetitionRange {
        Exactly(usize),
        AtLeast(usize),
        Bounded(usize, usize),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Literal;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Class;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Anchor;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct WordBoundary;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Group;

    let literal = Literal;
    let rep = Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir {
            kind: HirKind::Literal(literal),
            info: HirInfo::new(),
        }),
    };
    let result = Hir::repetition(rep);
    assert!(result.is_match_empty());
    assert!(result.is_any_anchored_start());
    assert!(result.is_any_anchored_end());
}

#[test]
fn test_repetition_non_empty() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    enum RepetitionKind {
        ZeroOrOne,
        ZeroOrMore,
        OneOrMore,
        Range(RepetitionRange),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum RepetitionRange {
        Exactly(usize),
        AtLeast(usize),
        Bounded(usize, usize),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Literal;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Class;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Anchor;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct WordBoundary;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Group;

    let literal = Literal;
    let rep = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(Hir {
            kind: HirKind::Literal(literal),
            info: HirInfo::new(),
        }),
    };
    let result = Hir::repetition(rep);
    assert!(!result.is_match_empty());
    assert!(!result.is_any_anchored_start());
    assert!(!result.is_any_anchored_end());
}

