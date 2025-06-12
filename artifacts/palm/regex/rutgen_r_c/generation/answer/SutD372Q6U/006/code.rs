// Answer 0

#[test]
fn test_parse_group_with_invalid_flags() {
    struct TestParser {
        pos: Cell<Position>,
        capture_index: Cell<u32>,
        octal: bool,
        pattern: &'static str,
        current_char: char,
        eof: bool,
    }

    impl TestParser {
        fn new(pattern: &'static str) -> Self {
            TestParser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                capture_index: Cell::new(0),
                octal: false,
                pattern,
                current_char: '(',
                eof: false,
            }
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn bump(&self) {
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: self.pos.get().line, column: self.pos.get().column + 1 });
        }

        fn span(&self) -> Span {
            Span::new(self.pos.get(), self.pos.get())
        }

        fn bump_if(&mut self, token: &str) -> bool {
            if token == "?" {
                self.bump();
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn parse_flags(&self) -> Result<ast::Flags> {
            Err(ast::Error { kind: ast::ErrorKind::FlagUnexpectedEof, pattern: self.pattern.to_string(), span: self.span() })
        }

        fn is_lookaround_prefix(&self) -> bool {
            false
        }
    }

    impl ParserI<'_, TestParser> {
        fn new(parser: TestParser) -> Self {
            ParserI { parser, pattern: parser.pattern }
        }
    }

    let parser = TestParser::new("(?i)");
    let parser_instance = ParserI::new(parser);
    let result = parser_instance.parse_group();

    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::RepetitionMissing);
    }
}

