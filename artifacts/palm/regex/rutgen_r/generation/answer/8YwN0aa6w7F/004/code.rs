// Answer 0

#[test]
fn test_has_subexprs_repetition() {
    struct TestRepetition;

    impl TestRepetition {
        fn new() -> Self {
            TestRepetition {}
        }
    }

    enum HirKind {
        Empty,
        Literal(char),
        Class(char),
        Anchor(char),
        WordBoundary(char),
        Group(Box<TestRepetition>),
        Repetition(Box<TestRepetition>),
        Concat(Box<TestRepetition>, Box<TestRepetition>),
        Alternation(Box<TestRepetition>, Box<TestRepetition>),
    }

    impl HirKind {
        pub fn has_subexprs(&self) -> bool {
            match *self {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => false,
                HirKind::Group(_)
                | HirKind::Repetition(_)
                | HirKind::Concat(_, _)
                | HirKind::Alternation(_, _) => true,
            }
        }
    }

    let repetition = HirKind::Repetition(Box::new(TestRepetition::new()));
    assert!(repetition.has_subexprs());
}

