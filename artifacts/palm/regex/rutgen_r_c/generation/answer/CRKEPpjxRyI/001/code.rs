// Answer 0

#[test]
fn test_parse_set_class_range_valid_literal_range() {
    struct MockParser;

    impl MockParser {
        fn bump_space(&self) {}
        fn is_eof(&self) -> bool { false }
        fn char(&self) -> char { '-' }
        fn peek_space(&self) -> Option<char> { Some('a') }
        fn parse_set_class_item(&self) -> Result<Primitive> {
            let span = Span::new(Position { offset: 0, line: 1, column: 1 },
                                  Position { offset: 1, line: 1, column: 2 });
            let lit1 = Literal { span: span.clone(), kind: ast::LiteralKind::Verbatim, c: 'a' };
            let lit2 = Literal { span: span, kind: ast::LiteralKind::Verbatim, c: 'b' };
            Ok(Primitive::Literal(lit1))  // First item is 'a'
        }
        fn error(&self, _span: &Span, _kind: ast::ErrorKind) -> Error {
            Error { kind: ast::ErrorKind::ClassRangeInvalid, pattern: String::new(), span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }) }
        }
    }

    let parser = ParserI { parser: MockParser, pattern: "[a-b]" };
    let result = parser.parse_set_class_range();
    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_range_invalid_range() {
    struct MockParser;

    impl MockParser {
        fn bump_space(&self) {}
        fn is_eof(&self) -> bool { false }
        fn char(&self) -> char { '-' }
        fn peek_space(&self) -> Option<char> { Some('a') }
        fn parse_set_class_item(&self) -> Result<Primitive> {
            let span = Span::new(Position { offset: 0, line: 1, column: 1 },
                                  Position { offset: 1, line: 1, column: 2 });
            let lit1 = Literal { span: span.clone(), kind: ast::LiteralKind::Verbatim, c: 'b' };
            let lit2 = Literal { span: span, kind: ast::LiteralKind::Verbatim, c: 'a' }; 
            Ok(Primitive::Literal(lit1))  // Invalid case: 'b' is greater than 'a'
        }
        fn error(&self, _span: &Span, _kind: ast::ErrorKind) -> Error {
            Error { kind: ast::ErrorKind::ClassRangeInvalid, pattern: String::new(), span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }) }
        }
    }

    let parser = ParserI { parser: MockParser, pattern: "[b-a]" };
    let result = parser.parse_set_class_range();
    assert!(result.is_err());
}

#[test]
fn test_parse_set_class_range_unclosed_class() {
    struct MockParser;

    impl MockParser {
        fn bump_space(&self) {}
        fn is_eof(&self) -> bool { false }
        fn char(&self) -> char { '-' }
        fn peek_space(&self) -> Option<char> { None }
        fn parse_set_class_item(&self) -> Result<Primitive> {
            Ok(Primitive::Literal(Literal { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }), kind: ast::LiteralKind::Verbatim, c: 'a' }))
        }
        fn unclosed_class_error(&self) -> Error {
            Error { kind: ast::ErrorKind::ClassUnclosed, pattern: String::new(), span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }) }
        }
    }

    let parser = ParserI { parser: MockParser, pattern: "[a-" }; // Missing closing bracket
    let result = parser.parse_set_class_range();
    assert!(result.is_err());
}

#[test]
fn test_parse_set_class_range_invalid_escape() {
    struct MockParser;

    impl MockParser {
        fn bump_space(&self) {}
        fn is_eof(&self) -> bool { false }
        fn char(&self) -> char { '-' }
        fn peek_space(&self) -> Option<char> { Some('a') }
        fn parse_set_class_item(&self) -> Result<Primitive> {
            Err(Error { kind: ast::ErrorKind::ClassEscapeInvalid, pattern: String::new(), span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }) })
        }
    }

    let parser = ParserI { parser: MockParser, pattern: "[*-" }; // Invalid escape with '*'
    let result = parser.parse_set_class_range();
    assert!(result.is_err());
}

