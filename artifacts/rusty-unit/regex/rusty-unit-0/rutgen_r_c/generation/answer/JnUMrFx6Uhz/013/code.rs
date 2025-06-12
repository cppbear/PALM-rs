// Answer 0

#[test]
fn test_parse_flags_dangling_negation() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn span_char(&self) -> Span {
            Span {
                start: Position { offset: self.pos, line: 1, column: 1 },
                end: Position { offset: self.pos + 1, line: 1, column: 1 },
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.input.iter().collect(), span }
        }
        
        fn parse_flags(&mut self) -> Result<ast::Flags> {
            let mut flags = ast::Flags {
                span: self.span_char(),
                items: vec![],
            };
            let mut last_was_negation = None;
            while self.char() != ':' && self.char() != ')' {
                if self.char() == '-' {
                    last_was_negation = Some(self.span_char());
                    let item = ast::FlagsItem {
                        span: self.span_char(),
                        kind: ast::FlagsItemKind::Negation,
                    };
                    if let Some(i) = flags.add_item(item) {
                        return Err(self.error(
                            self.span_char(),
                            ast::ErrorKind::FlagRepeatedNegation { original: flags.items[i].span },
                        ));
                    }
                } else {
                    last_was_negation = None;
                    let item = ast::FlagsItem {
                        span: self.span_char(),
                        kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                    };
                    if let Some(i) = flags.add_item(item) {
                        return Err(self.error(
                            self.span_char(),
                            ast::ErrorKind::FlagDuplicate { original: flags.items[i].span },
                        ));
                    }
                }
                if !self.bump() {
                    return Err(self.error(
                        self.span_char(),
                        ast::ErrorKind::FlagUnexpectedEof,
                    ));
                }
            }
            if let Some(span) = last_was_negation {
                return Err(self.error(span, ast::ErrorKind::FlagDanglingNegation));
            }
            flags.span.end = flags.span.end; // dummy assignment for completion
            Ok(flags)
        }
    }

    let mut parser = MockParser::new("-:"); // Input that triggers dangling negation
    let result = parser.parse_flags();
    assert!(result.is_err());
    
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::FlagDanglingNegation);
    }
}

