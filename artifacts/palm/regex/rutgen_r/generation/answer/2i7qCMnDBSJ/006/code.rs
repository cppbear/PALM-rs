// Answer 0

#[test]
fn test_ast_empty() {
    struct AstEmpty;

    impl Ast for AstEmpty {
        fn has_subexprs(&self) -> bool {
            false
        }
    }

    let ast = AstEmpty;
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_ast_flags() {
    struct AstFlags;

    impl Ast for AstFlags {
        fn has_subexprs(&self) -> bool {
            false
        }
    }

    let ast = AstFlags;
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_ast_literal() {
    struct AstLiteral;

    impl Ast for AstLiteral {
        fn has_subexprs(&self) -> bool {
            false
        }
    }

    let ast = AstLiteral;
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_ast_dot() {
    struct AstDot;

    impl Ast for AstDot {
        fn has_subexprs(&self) -> bool {
            false
        }
    }

    let ast = AstDot;
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_ast_assertion() {
    struct AstAssertion;

    impl Ast for AstAssertion {
        fn has_subexprs(&self) -> bool {
            false
        }
    }

    let ast = AstAssertion;
    assert_eq!(ast.has_subexprs(), false);
}

