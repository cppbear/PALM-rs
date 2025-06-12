// Answer 0

fn test_c_alternate_empty_subexpression() {
    struct DummyHir {
        kind: syntax::hir::HirKind,
    }
    
    let mut compiler = Compiler::new();
    
    let empty_hir = DummyHir {
        kind: syntax::hir::HirKind::Empty,
    };
    
    let valid_hir = DummyHir {
        kind: syntax::hir::HirKind::Literal(syntax::hir::Literal::Unicode('a')),
    };
    
    let exprs = vec![
        &empty_hir, // This simulates an empty sub-expression, which should cause a panic
        &valid_hir,
    ];
    
    let result = compiler.c_alternate(&exprs);
    
    assert_eq!(result, Err(Error::Syntax(
        "alternations cannot currently contain \
         empty sub-expressions".to_string()
    )));
}

fn test_c_alternate_two_valid_expressions() {
    struct DummyHir {
        kind: syntax::hir::HirKind,
    }
    
    let mut compiler = Compiler::new();
    
    let first_hir = DummyHir {
        kind: syntax::hir::HirKind::Literal(syntax::hir::Literal::Unicode('a')),
    };
    
    let second_hir = DummyHir {
        kind: syntax::hir::HirKind::Literal(syntax::hir::Literal::Unicode('b')),
    };
    
    let exprs = vec![
        &first_hir, // valid expression
        &second_hir, // valid expression
    ];
    
    // Here we want to ensure that it does not panic and returns Ok result
    let result = compiler.c_alternate(&exprs);
    
    assert!(result.is_ok());
}

