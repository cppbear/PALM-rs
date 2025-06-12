// Answer 0

#[test]
fn test_compile_empty() {
    struct TestHir {
        kind: syntax::hir::HirKind,
    }

    let mut compiler = Compiler::new();
    let expr = TestHir {
        kind: syntax::hir::HirKind::Empty,
    };
    let result = compiler.c(&expr);

    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.entry, 0);
    assert!(matches!(patch.hole, Hole::None));
}

#[test]
fn test_compile_unicode_literal() {
    struct TestHir {
        kind: syntax::hir::HirKind,
    }

    let mut compiler = Compiler::new();
    let expr = TestHir {
        kind: syntax::hir::HirKind::Literal(syntax::hir::Literal::Unicode('a')),
    };
    let result = compiler.c(&expr);

    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.entry, 0);
    assert!(matches!(patch.hole, Hole::None));
}

#[test]
fn test_compile_byte_literal() {
    struct TestHir {
        kind: syntax::hir::HirKind,
    }

    let mut compiler = Compiler::new().bytes(true);
    let expr = TestHir {
        kind: syntax::hir::HirKind::Literal(syntax::hir::Literal::Byte(b'a')),
    };
    let result = compiler.c(&expr);

    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.entry, 0);
    assert!(matches!(patch.hole, Hole::None));
}

#[test]
fn test_compile_word_boundary() {
    struct TestHir {
        kind: syntax::hir::HirKind,
    }

    let mut compiler = Compiler::new();
    let expr = TestHir {
        kind: syntax::hir::HirKind::WordBoundary(syntax::hir::WordBoundary::Unicode),
    };
    let result = compiler.c(&expr);

    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.entry, 0);
    assert!(!matches!(patch.hole, Hole::None));
}

