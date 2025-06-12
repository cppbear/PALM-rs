// Answer 0

#[test]
fn test_parse_set_class_range_with_lit_range_valid() {
    let span1 = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 });
    let span2 = Span::new(Position { offset: 2, line: 1, column: 3 }, Position { offset: 3, line: 1, column: 4 });
    
    let prim1 = Primitive::Literal(Literal { span: span1.clone(), kind: ast::LiteralKind::Verbatim, c: 'a' });
    let prim2 = Primitive::Literal(Literal { span: span2.clone(), kind: ast::LiteralKind::Verbatim, c: 'b' });

    let parser = Parser { /* initialize parser state */ };
    let parser_i = ParserI { parser: &parser, pattern: "a-b" };

    parser_i.parse_set_class_item = || Ok(prim1.clone());
    parser_i.is_eof = || false;
    parser_i.char = || '-';
    parser_i.peek_space = || Some(']');

    parser_i.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_with_literal_implicit_range() {
    let span1 = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 });
    let span2 = Span::new(Position { offset: 2, line: 1, column: 3 }, Position { offset: 3, line: 1, column: 4 });
    
    let prim1 = Primitive::Literal(Literal { span: span1.clone(), kind: ast::LiteralKind::Verbatim, c: 'c' });
    let prim2 = Primitive::Literal(Literal { span: span2.clone(), kind: ast::LiteralKind::Verbatim, c: 'd' });

    let parser = Parser { /* initialize parser state */ };
    let parser_i = ParserI { parser: &parser, pattern: "c-d" };

    parser_i.parse_set_class_item = || Ok(prim1.clone());
    parser_i.is_eof = || false;
    parser_i.char = || '-';
    parser_i.peek_space = || Some(']');

    parser_i.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_with_invalid_range() {
    let span1 = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 });
    let span2 = Span::new(Position { offset: 3, line: 1, column: 4 }, Position { offset: 2, line: 1, column: 3 });
    
    let prim1 = Primitive::Literal(Literal { span: span1.clone(), kind: ast::LiteralKind::Verbatim, c: 'd' });
    let prim2 = Primitive::Literal(Literal { span: span2.clone(), kind: ast::LiteralKind::Verbatim, c: 'c' });

    let parser = Parser { /* initialize parser state */ };
    let parser_i = ParserI { parser: &parser, pattern: "d-c" };

    parser_i.parse_set_class_item = || Ok(prim1.clone());
    parser_i.is_eof = || false;
    parser_i.char = || '-';
    parser_i.peek_space = || Some(']');

    parser_i.parse_set_class_range();
}

