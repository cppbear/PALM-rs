// Answer 0

#[test]
fn test_has_subexprs_empty() {
    struct AstEmpty;
    impl Ast for AstEmpty {}

    let ast = Ast::Empty(AstEmpty);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_flags() {
    struct AstFlags;
    impl Ast for AstFlags {}

    let ast = Ast::Flags(AstFlags);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_literal() {
    struct AstLiteral;
    impl Ast for AstLiteral {}

    let ast = Ast::Literal(AstLiteral);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_dot() {
    struct AstDot;
    impl Ast for AstDot {}

    let ast = Ast::Dot(AstDot);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_assertion() {
    struct AstAssertion;
    impl Ast for AstAssertion {}

    let ast = Ast::Assertion(AstAssertion);
    assert_eq!(ast.has_subexprs(), false);
}

