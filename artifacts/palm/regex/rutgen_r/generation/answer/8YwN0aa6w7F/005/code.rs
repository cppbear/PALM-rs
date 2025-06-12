// Answer 0

#[test]
fn test_hirkind_has_subexprs_word_boundary() {
    struct HirKindDummy {
        kind: HirKind,
    }

    impl HirKindDummy {
        fn has_subexprs(&self) -> bool {
            match self.kind {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => false,
                HirKind::Group(_)
                | HirKind::Repetition(_)
                | HirKind::Concat(_)
                | HirKind::Alternation(_) => true,
            }
        }
    }

    let word_boundary = HirKindDummy { kind: HirKind::WordBoundary(()) };
    assert_eq!(word_boundary.has_subexprs(), false);
}

#[test]
fn test_hirkind_has_subexprs_literal() {
    struct HirKindDummy {
        kind: HirKind,
    }

    impl HirKindDummy {
        fn has_subexprs(&self) -> bool {
            match self.kind {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => false,
                HirKind::Group(_)
                | HirKind::Repetition(_)
                | HirKind::Concat(_)
                | HirKind::Alternation(_) => true,
            }
        }
    }

    let literal = HirKindDummy { kind: HirKind::Literal("a") };
    assert_eq!(literal.has_subexprs(), false);
}

#[test]
fn test_hirkind_has_subexprs_class() {
    struct HirKindDummy {
        kind: HirKind,
    }

    impl HirKindDummy {
        fn has_subexprs(&self) -> bool {
            match self.kind {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => false,
                HirKind::Group(_)
                | HirKind::Repetition(_)
                | HirKind::Concat(_)
                | HirKind::Alternation(_) => true,
            }
        }
    }

    let class = HirKindDummy { kind: HirKind::Class(vec!['a', 'b']) };
    assert_eq!(class.has_subexprs(), false);
}

#[test]
fn test_hirkind_has_subexprs_anchor() {
    struct HirKindDummy {
        kind: HirKind,
    }

    impl HirKindDummy {
        fn has_subexprs(&self) -> bool {
            match self.kind {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => false,
                HirKind::Group(_)
                | HirKind::Repetition(_)
                | HirKind::Concat(_)
                | HirKind::Alternation(_) => true,
            }
        }
    }

    let anchor = HirKindDummy { kind: HirKind::Anchor(AnchorType::Start) };
    assert_eq!(anchor.has_subexprs(), false);
}

#[test]
fn test_hirkind_has_subexprs_empty() {
    struct HirKindDummy {
        kind: HirKind,
    }

    impl HirKindDummy {
        fn has_subexprs(&self) -> bool {
            match self.kind {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => false,
                HirKind::Group(_)
                | HirKind::Repetition(_)
                | HirKind::Concat(_)
                | HirKind::Alternation(_) => true,
            }
        }
    }

    let empty = HirKindDummy { kind: HirKind::Empty };
    assert_eq!(empty.has_subexprs(), false);
}

