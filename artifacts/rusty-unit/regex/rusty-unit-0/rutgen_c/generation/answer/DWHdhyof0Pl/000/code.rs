// Answer 0

#[test]
fn test_c_repeat_one_or_more_greedy() {
    struct TestHir {
        kind: syntax::hir::HirKind,
    }

    let mut compiler = Compiler::new();
    let expr = TestHir {
        kind: syntax::hir::HirKind::Repetition(Box::new(syntax::hir::Repetition {
            expr: Box::new(TestHir {
                kind: syntax::hir::HirKind::Literal(syntax::hir::Literal::Unicode('a')),
            }),
            min: 1,
            max: None,
            greedy: true,
        })),
    };

    let result = compiler.c_repeat_one_or_more(&expr, true);

    assert!(result.is_ok());
}

#[test]
fn test_c_repeat_one_or_more_nongreedy() {
    struct TestHir {
        kind: syntax::hir::HirKind,
    }

    let mut compiler = Compiler::new();
    let expr = TestHir {
        kind: syntax::hir::HirKind::Repetition(Box::new(syntax::hir::Repetition {
            expr: Box::new(TestHir {
                kind: syntax::hir::HirKind::Literal(syntax::hir::Literal::Unicode('a')),
            }),
            min: 1,
            max: None,
            greedy: false,
        })),
    };

    let result = compiler.c_repeat_one_or_more(&expr, false);

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_c_repeat_one_or_more_invalid() {
    struct TestHir {
        kind: syntax::hir::HirKind,
    }

    let mut compiler = Compiler::new();
    let expr = TestHir {
        kind: syntax::hir::HirKind::Repetition(Box::new(syntax::hir::Repetition {
            expr: Box::new(TestHir {
                kind: syntax::hir::HirKind::Literal(syntax::hir::Literal::Unicode('b')),
            }),
            min: 0,
            max: None,
            greedy: true,
        })),
    };

    compiler.c_repeat_one_or_more(&expr, true).unwrap();
}

