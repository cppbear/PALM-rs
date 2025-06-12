// Answer 0

#[test]
fn test_into_class_literal_valid() {
    let span = Span { start: 0, end: 1 };
    let literal = ast::Literal::Unicode('a');
    let primitive = Primitive::Literal(ast::Literal { span: span.clone(), kind: ast::LiteralKind::Unicode, c: 'a' });
    let parser = ParserI { parser: (), pattern: "a" };

    let result = primitive.into_class_literal(&parser);
    assert_eq!(result, Ok(literal));
}

#[test]
fn test_into_class_literal_invalid() {
    let span = Span { start: 0, end: 1 };
    let assertion = Primitive::Assertion(ast::Assertion { span: span.clone(), kind: ast::AssertionKind::WordBoundary });
    let parser = ParserI { parser: (), pattern: "b" };

    let result = assertion.into_class_literal(&parser);
    assert!(result.is_err());

    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassRangeLiteral);
        assert_eq!(err.pattern, "b");
        assert_eq!(err.span, span);
    }
}

