// Answer 0

#[test]
fn test_prefixes_with_concat_non_empty() {
    use regex_syntax::hir::{Hir, HirKind, Class, Literal, Group, Repetition, RepetitionKind, RepetitionRange, Anchor, Alternation};
    use regex_syntax::Literals;

    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new(kind: HirKind) -> Self {
            TestHir { kind }
        }
    }

    fn create_test_hir_concat() -> Hir {
        let expr1 = TestHir::new(HirKind::Literal(hir::Literal::Unicode('a')));
        let expr2 = TestHir::new(HirKind::Literal(hir::Literal::Unicode('b')));
        Hir::concat(vec![expr1, expr2])
    }

    let expr = create_test_hir_concat();
    let mut literals = Literals::new();

    prefixes(&expr, &mut literals);

    assert!(!literals.is_empty());
}

#[test]
fn test_prefixes_with_concat_multiple() {
    use regex_syntax::hir::{Hir, HirKind, Class, Literal, Group, Repetition, RepetitionKind, RepetitionRange, Anchor, Alternation};
    use regex_syntax::Literals;

    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new(kind: HirKind) -> Self {
            TestHir { kind }
        }
    }

    fn create_test_hir_concat() -> Hir {
        let expr1 = TestHir::new(HirKind::Literal(hir::Literal::Unicode('a')));
        let expr2 = TestHir::new(HirKind::Literal(hir::Literal::Unicode('b')));
        let expr3 = TestHir::new(HirKind::Literal(hir::Literal::Unicode('c')));
        Hir::concat(vec![expr1, expr2, expr3])
    }

    let expr = create_test_hir_concat();
    let mut literals = Literals::new();

    prefixes(&expr, &mut literals);

    assert!(!literals.is_empty());
} 

#[test]
fn test_prefixes_repetition_range() {
    use regex_syntax::hir::{Hir, HirKind, Repetition, RepetitionKind, RepetitionRange};
    use regex_syntax::Literals;

    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new(kind: HirKind) -> Self {
            TestHir { kind }
        }
    }

    fn create_test_hir_repetition() -> Hir {
        let expr = TestHir::new(HirKind::Literal(hir::Literal::Unicode('a')));
        Hir::repetition(Box::new(expr), RepetitionKind::Range(RepetitionRange::Bounded(2, 4)))
    }

    let expr = create_test_hir_repetition();
    let mut literals = Literals::new();

    prefixes(&expr, &mut literals);

    assert!(!literals.is_empty());
} 

#[test]
#[should_panic]
fn test_prefixes_with_empty_concat() {
    use regex_syntax::hir::{Hir, HirKind, Literal, Group};
    use regex_syntax::Literals;

    let expr = Hir::concat(vec![]);
    let mut literals = Literals::new();

    prefixes(&expr, &mut literals);
} 

#[test]
fn test_prefixes_with_alternation() {
    use regex_syntax::hir::{Hir, HirKind, Alternation, Literal};
    use regex_syntax::Literals;

    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new(kind: HirKind) -> Self {
            TestHir { kind }
        }
    }

    fn create_test_hir_alternation() -> Hir {
        let expr1 = TestHir::new(HirKind::Literal(hir::Literal::Unicode('x')));
        let expr2 = TestHir::new(HirKind::Literal(hir::Literal::Unicode('y')));
        Hir::alternation(vec![expr1, expr2])
    }

    let expr = create_test_hir_alternation();
    let mut literals = Literals::new();

    prefixes(&expr, &mut literals);

    assert!(!literals.is_empty());
}

