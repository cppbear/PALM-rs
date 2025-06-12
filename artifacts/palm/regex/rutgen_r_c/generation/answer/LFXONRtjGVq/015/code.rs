// Answer 0

#[test]
fn test_parse_counted_repetition_invalid_range() {
    struct DummyParser {
        position: Position,
        current_char: char,
        eof: bool,
        concat: ast::Concat,
    }

    impl DummyParser {
        fn new() -> Self {
            Self {
                position: Position { offset: 0, line: 1, column: 1 },
                current_char: '{',
                eof: false,
                concat: ast::Concat { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }), asts: vec![] }
            }
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn bump(&mut self) {
            self.position.offset += 1;
            self.position.column += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump();
            true
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn parse_decimal(&self) -> Result<u32> {
            Ok(1)
        }

        fn pop_ast(&mut self) -> Option<Ast> {
            Some(Ast::Empty(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 })))
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }) }
        }
    }

    let mut parser = DummyParser::new();
    parser.concat.asts.push(parser.pop_ast().unwrap());
    parser.concat.asts.push(parser.pop_ast().unwrap());
    parser.bump_and_bump_space(); // Move past the '{'

    // Simulating invalid range: this will trigger the panic condition when 'is_valid' is checked
    let result = parser.parse_counted_repetition(parser.concat);

    assert!(result.is_err());
    if let Err(ref error) = result {
        assert_eq!(error.kind, ast::ErrorKind::RepetitionCountInvalid);
    }
}

