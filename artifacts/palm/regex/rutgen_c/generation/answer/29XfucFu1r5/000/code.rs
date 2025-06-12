// Answer 0

#[test]
fn test_into_class_set_item_literal() {
    let span = Span { start: 0, end: 1 };
    let literal = ast::Literal { span: span.clone(), kind: ast::LiteralKind::Unicode('a'), c: 'a' };
    let primitive = Primitive::Literal(literal.clone());
    let parser = ParserI::new((), "a");

    let result = primitive.into_class_set_item(&parser);
    assert!(result.is_ok());
    if let Ok(ast::ClassSetItem::Literal(lit)) = result {
        assert_eq!(lit, literal);
    } else {
        panic!("Expected Literal");
    }
}

#[test]
fn test_into_class_set_item_perl() {
    let span = Span { start: 0, end: 1 };
    let perl_class = ast::ClassPerl { span: span.clone(), kind: ast::ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(perl_class.clone());
    let parser = ParserI::new((), ".*");

    let result = primitive.into_class_set_item(&parser);
    assert!(result.is_ok());
    if let Ok(ast::ClassSetItem::Perl(cls)) = result {
        assert_eq!(cls, perl_class);
    } else {
        panic!("Expected Perl Class");
    }
}

#[test]
fn test_into_class_set_item_unicode() {
    let span = Span { start: 0, end: 1 };
    let unicode_class = ast::ClassUnicode { span: span.clone(), negated: false, kind: ast::ClassUnicodeKind::Letter };
    let primitive = Primitive::Unicode(unicode_class.clone());
    let parser = ParserI::new((), ".*");

    let result = primitive.into_class_set_item(&parser);
    assert!(result.is_ok());
    if let Ok(ast::ClassSetItem::Unicode(cls)) = result {
        assert_eq!(cls, unicode_class);
    } else {
        panic!("Expected Unicode Class");
    }
}

#[test]
fn test_into_class_set_item_invalid() {
    let span = Span { start: 0, end: 1 };
    let primitive = Primitive::Dot(span.clone());
    let parser = ParserI::new((), ".*");

    let result = primitive.into_class_set_item(&parser);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassEscapeInvalid);
    } else {
        panic!("Expected an error");
    }
}

