// Answer 0

#[test]
fn test_into_class_literal_with_unicode() {
    let span = Span { start: 0, end: 1 };
    let lit = ast::Literal::Unicode('a');
    let primitive = Primitive::Literal(ast::Literal {
        span: span,
        kind: ast::LiteralKind::Unicode,
        c: 'a',
    });
    let parser = ParserI {
        parser: (),
        pattern: "a",
    };
    
    let result = primitive.into_class_literal(&parser);
    assert!(result.is_ok());
    if let Ok(literal) = result {
        assert_eq!(literal, lit);
    }
}

#[test]
fn test_into_class_literal_with_byte() {
    let span = Span { start: 0, end: 1 };
    let lit = ast::Literal::Byte(97);
    let primitive = Primitive::Literal(ast::Literal {
        span: span,
        kind: ast::LiteralKind::Byte,
        c: 'a',
    });
    let parser = ParserI {
        parser: (),
        pattern: "a",
    };
    
    let result = primitive.into_class_literal(&parser);
    assert!(result.is_ok());
    if let Ok(literal) = result {
        assert_eq!(literal, lit);
    }
}

