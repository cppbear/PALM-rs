// Answer 0

#[test]
fn test_prefixes_one_or_more_repetition() {
    use regex_syntax::hir::{self, Hir, HirKind, Literal, Repetition, RepetitionKind};
    use regex_syntax::Literals;

    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new(kind: HirKind) -> Self {
            TestHir { kind }
        }

        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut lits = Literals::new();

    // Create a repetition with kind OneOrMore and a simple literal
    let inner_literal = TestHir::new(HirKind::Literal(hir::Literal::Unicode('a')));
    let repetition = Repetition {
        hir: Box::new(inner_literal),
        kind: RepetitionKind::OneOrMore,
        greedy: true,
    };

    let expr = TestHir::new(HirKind::Repetition(repetition));

    prefixes(&expr, &mut lits);

    // Assert expected results in lit
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_one_or_more_repetition_empty_literaks() {
    use regex_syntax::hir::{self, HirKind, Repetition, RepetitionKind};
    use regex_syntax::Literals;

    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new(kind: HirKind) -> Self {
            TestHir { kind }
        }

        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut lits = Literals::new();

    // Create a repetition with kind OneOrMore and an empty literal
    let inner_literal = TestHir::new(HirKind::Literal(hir::Literal::Unicode('\0')));
    let repetition = Repetition {
        hir: Box::new(inner_literal),
        kind: RepetitionKind::OneOrMore,
        greedy: true,
    };

    let expr = TestHir::new(HirKind::Repetition(repetition));

    prefixes(&expr, &mut lits);

    // Assert expected results in lit
    assert!(!lits.is_empty());
}

