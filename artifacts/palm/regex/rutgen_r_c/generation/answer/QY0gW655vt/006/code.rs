// Answer 0

#[test]
fn test_c_alternate_two_empty_expressions() {
    struct TestHir {
        kind: hir::HirKind,
    }

    impl TestHir {
        fn new(kind: hir::HirKind) -> Self {
            TestHir { kind }
        }
    }

    let mut compiler = Compiler::new();
    let first_expr = TestHir::new(hir::HirKind::Empty);
    let second_expr = TestHir::new(hir::HirKind::Empty);
    let exprs = vec![first_expr, second_expr];

    let result = compiler.c_alternate(&exprs);

    assert_eq!(result, Err(Error::Syntax("alternations cannot currently contain empty sub-expressions".to_string())));
}

#[test]
fn test_c_alternate_two_valid_expressions() {
    struct TestHir {
        kind: hir::HirKind,
    }

    impl TestHir {
        fn new(kind: hir::HirKind) -> Self {
            TestHir { kind }
        }
    }

    let mut compiler = Compiler::new();
    let first_expr = TestHir::new(hir::HirKind::Literal(hir::Literal::Unicode('a')));
    let second_expr = TestHir::new(hir::HirKind::Literal(hir::Literal::Unicode('b')));
    let exprs = vec![first_expr, second_expr];

    let result = compiler.c_alternate(&exprs);

    assert!(result.is_ok());
}

#[test]
fn test_c_alternate_two_empty_with_previous_filled() {
    struct TestHir {
        kind: hir::HirKind,
    }

    impl TestHir {
        fn new(kind: hir::HirKind) -> Self {
            TestHir { kind }
        }
    }

    let mut compiler = Compiler::new();
    
    // Simulate filling previous holes
    compiler.push_split_hole();
    
    let first_expr = TestHir::new(hir::HirKind::Empty);
    let second_expr = TestHir::new(hir::HirKind::Empty);
    let exprs = vec![first_expr, second_expr];

    let result = compiler.c_alternate(&exprs);

    assert_eq!(result, Err(Error::Syntax("alternations cannot currently contain empty sub-expressions".to_string())));
}

