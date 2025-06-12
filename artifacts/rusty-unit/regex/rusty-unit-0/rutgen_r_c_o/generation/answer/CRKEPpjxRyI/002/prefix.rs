// Answer 0

#[test]
fn test_parse_set_class_range_no_range_after_item() {
    let parser = ParserI {
        parser: Parser::new(),
        pattern: "a".to_string().as_str(),
    };
    let _ = parser.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_eof_after_item() {
    let parser = ParserI {
        parser: Parser::new(),
        pattern: "a-".to_string().as_str(),
    };
    let _ = parser.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_invalid_item() {
    let parser = ParserI {
        parser: Parser::new(),
        pattern: "a-".to_string().as_str(),
    };
    let _ = parser.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_with_invalid_range() {
    let prim1 = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 });
    let prim2 = Span::new(Position { offset: 2, line: 1, column: 3 }, Position { offset: 3, line: 1, column: 4 });
    let parser = ParserI {
        parser: Parser::new(),
        pattern: "a-z".to_string().as_str(),
    };
    parser.bump_space();
    let _ = parser.parse_set_class_range();
}

