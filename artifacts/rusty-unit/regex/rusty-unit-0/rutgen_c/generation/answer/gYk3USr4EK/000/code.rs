// Answer 0

#[test]
fn test_union_prefixes_success() {
    struct TestHir {
        kind: hir::HirKind,
    }

    impl TestHir {
        fn new_unicode(c: char) -> Self {
            TestHir { kind: hir::HirKind::Literal(hir::Literal::Unicode(c)) }
        }
    }

    let mut literals = Literals::empty();
    let test_expr = TestHir::new_unicode('a');

    assert!(literals.union_prefixes(&test_expr));
}

#[test]
fn test_union_prefixes_empty() {
    struct TestHir {
        kind: hir::HirKind,
    }

    impl TestHir {
        fn new_empty() -> Self {
            TestHir { kind: hir::HirKind::Literal(hir::Literal::Unicode('\0')) }
        }
    }

    let mut literals = Literals::empty();
    let test_expr = TestHir::new_empty();

    assert!(!literals.union_prefixes(&test_expr));
}

#[test]
fn test_union_prefixes_exceeds_limit() {
    struct TestHir {
        kind: hir::HirKind,
    }

    impl TestHir {
        fn new_unicode(c: char) -> Self {
            TestHir { kind: hir::HirKind::Literal(hir::Literal::Unicode(c)) }
        }
    }

    let mut literals = Literals::empty();
    literals.set_limit_size(1);
    literals.add(Literal::Unicode('a'));
    let test_expr = TestHir::new_unicode('b');

    assert!(!literals.union_prefixes(&test_expr));
}

#[test]
fn test_union_prefixes_multiple() {
    struct TestHir {
        kind: hir::HirKind,
    }

    impl TestHir {
        fn new_concat(exprs: Vec<hir::HirKind>) -> Self {
            TestHir { kind: hir::HirKind::Concat(exprs) }
        }
    }

    let mut literals = Literals::empty();
    let test_expr = TestHir::new_concat(vec![
        hir::Literal::Unicode('a'),
        hir::Literal::Unicode('b'),
    ]);

    assert!(literals.union_prefixes(&test_expr));
}

