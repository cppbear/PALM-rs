// Answer 0

#[test]
fn test_parse_set_class_range_valid_range() {
    let start_pos1 = Position { offset: 0, line: 1, column: 1 };
    let start_pos2 = Position { offset: 2, line: 1, column: 3 };
    let span1 = Span::new(start_pos1, start_pos1);
    let span2 = Span::new(start_pos2, start_pos2);
    let literal1 = Literal { span: span1, kind: ast::LiteralKind::Verbatim, c: 'a' };
    let literal2 = Literal { span: span2, kind: ast::LiteralKind::Verbatim, c: 'b' };

    let parser_instance = ParserI {
        parser: Parser { /* initialization details omitted */ },
        pattern: "[a-b]",
    };

    parser_instance.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_singleton_range() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(start_pos, start_pos);
    let literal = Literal { span: span, kind: ast::LiteralKind::Verbatim, c: 'a' };

    let parser_instance = ParserI {
        parser: Parser { /* initialization details omitted */ },
        pattern: "[a-a]",
    };

    parser_instance.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_invalid_range() {
    let start_pos1 = Position { offset: 0, line: 1, column: 1 };
    let start_pos2 = Position { offset: 2, line: 1, column: 3 };
    let span1 = Span::new(start_pos1, start_pos1);
    let span2 = Span::new(start_pos2, start_pos2);
    let literal1 = Literal { span: span1, kind: ast::LiteralKind::Verbatim, c: 'b' };
    let literal2 = Literal { span: span2, kind: ast::LiteralKind::Verbatim, c: 'a' };

    let parser_instance = ParserI {
        parser: Parser { /* initialization details omitted */ },
        pattern: "[b-a]",
    };

    parser_instance.parse_set_class_range();
}

#[test]
#[should_panic]
fn test_parse_set_class_range_empty_class() {
    let parser_instance = ParserI {
        parser: Parser { /* initialization details omitted */ },
        pattern: "[]",
    };

    parser_instance.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_with_negation() {
    let start_pos1 = Position { offset: 0, line: 1, column: 1 };
    let start_pos2 = Position { offset: 3, line: 1, column: 4 };
    let span1 = Span::new(start_pos1, start_pos1);
    let span2 = Span::new(start_pos2, start_pos2);
    let literal1 = Literal { span: span1, kind: ast::LiteralKind::Verbatim, c: 'c' };
    let literal2 = Literal { span: span2, kind: ast::LiteralKind::Verbatim, c: 'd' };

    let parser_instance = ParserI {
        parser: Parser { /* initialization details omitted */ },
        pattern: "[c-d]",
    };

    parser_instance.parse_set_class_range();
}

