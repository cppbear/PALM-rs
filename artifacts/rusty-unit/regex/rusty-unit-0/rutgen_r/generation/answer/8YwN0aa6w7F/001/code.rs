// Answer 0

#[test]
fn test_has_subexprs_alternation() {
    struct Alternation;
    enum HirKind {
        Empty,
        Literal(String),
        Class(String),
        Anchor(String),
        WordBoundary(String),
        Group(Vec<HirKind>),
        Repetition(Box<HirKind>),
        Concat(Vec<HirKind>),
        Alternation(Vec<HirKind>),
    }

    impl Alternation {
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

    let alternation_expr = HirKind::Alternation(vec![
        HirKind::Literal("a".to_string()),
        HirKind::Literal("b".to_string()),
    ]);

    assert!(alternation_expr.has_subexprs());
}

