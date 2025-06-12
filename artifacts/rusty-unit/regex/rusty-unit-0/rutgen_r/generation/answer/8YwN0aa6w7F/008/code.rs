// Answer 0

#[test]
fn test_has_subexprs_literal() {
    struct HirKind {
        value: String,
    }
    
    impl HirKind {
        fn literal(value: &str) -> HirKind {
            HirKind { value: value.to_string() }
        }

        fn has_subexprs(&self) -> bool {
            match *self {
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

    let literal_hir = HirKind::literal("test");
    assert_eq!(literal_hir.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_word_boundary() {
    struct HirKind;

    impl HirKind {
        fn word_boundary() -> HirKind {
            HirKind
        }

        fn has_subexprs(&self) -> bool {
            match *self {
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

    let word_boundary_hir = HirKind::word_boundary();
    assert_eq!(word_boundary_hir.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_class() {
    struct HirKind;

    impl HirKind {
        fn class() -> HirKind {
            HirKind
        }

        fn has_subexprs(&self) -> bool {
            match *self {
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

    let class_hir = HirKind::class();
    assert_eq!(class_hir.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_anchor() {
    struct HirKind;

    impl HirKind {
        fn anchor() -> HirKind {
            HirKind
        }

        fn has_subexprs(&self) -> bool {
            match *self {
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

    let anchor_hir = HirKind::anchor();
    assert_eq!(anchor_hir.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_empty() {
    struct HirKind;

    impl HirKind {
        fn empty() -> HirKind {
            HirKind
        }

        fn has_subexprs(&self) -> bool {
            match *self {
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

    let empty_hir = HirKind::empty();
    assert_eq!(empty_hir.has_subexprs(), false);
}

