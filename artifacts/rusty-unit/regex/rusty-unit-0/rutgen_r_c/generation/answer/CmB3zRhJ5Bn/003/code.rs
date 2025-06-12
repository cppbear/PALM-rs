// Answer 0

#[test]
fn test_into_ast_dot() {
    // Define Span structure that will be used
    let span = Span { start: Position(0), end: Position(1) };
    
    // Create a Primitive::Dot instance
    let primitive = Primitive::Dot(span.clone());
    
    // Call the into_ast function
    let ast_result = primitive.into_ast();
    
    // Check that the result is an Ast::Dot with the correct span
    if let Ast::Dot(result_span) = ast_result {
        assert_eq!(result_span, span);
    } else {
        panic!("Expected Ast::Dot, but got a different variant");
    }
}

#[test]
fn test_into_ast_literal() {
    // Define Span structure that will be used
    let span = Span { start: Position(2), end: Position(3) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };

    // Create a Primitive::Literal instance
    let primitive = Primitive::Literal(literal.clone());

    // Call the into_ast function
    let ast_result = primitive.into_ast();

    // Check that the result is an Ast::Literal with the correct literal
    if let Ast::Literal(result_literal) = ast_result {
        assert_eq!(result_literal, literal);
    } else {
        panic!("Expected Ast::Literal, but got a different variant");
    }
}

#[test]
fn test_into_ast_assertion() {
    // Define Span structure that will be used
    let span = Span { start: Position(4), end: Position(5) };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::ZeroWidth };

    // Create a Primitive::Assertion instance
    let primitive = Primitive::Assertion(assertion.clone());

    // Call the into_ast function
    let ast_result = primitive.into_ast();

    // Check that the result is an Ast::Assertion with the correct assertion
    if let Ast::Assertion(result_assertion) = ast_result {
        assert_eq!(result_assertion, assertion);
    } else {
        panic!("Expected Ast::Assertion, but got a different variant");
    }
}

