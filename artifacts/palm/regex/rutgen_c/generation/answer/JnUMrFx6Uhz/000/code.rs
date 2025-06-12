// Answer 0

#[test]
fn test_parse_flags_success() {
    struct TestParser {
        pattern: String,
        current_index: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                current_index: 0,
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.current_index).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.current_index < self.pattern.len() - 1 {
                self.current_index += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span {
                start: Position { offset: 0, line: 1, column: 1 },
                end: Position { offset: self.current_index, line: 1, column: self.current_index + 1 },
            }
        }

        fn span_char(&self) -> Span {
            self.span()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }

        fn parse_flag(&self) -> Result<ast::Flag> {
            match self.char() {
                'i' => Ok(ast::Flag::CaseInsensitive),
                'm' => Ok(ast::Flag::MultiLine),
                's' => Ok(ast::Flag::DotMatchesNewLine),
                'U' => Ok(ast::Flag::SwapGreed),
                'u' => Ok(ast::Flag::Unicode),
                'x' => Ok(ast::Flag::IgnoreWhitespace),
                _ => Err(self.error(self.span_char(), ast::ErrorKind::FlagUnrecognized)),
            }
        }

        fn pos(&self) -> Position {
            Position { offset: self.current_index, line: 1, column: self.current_index + 1 }
        }
    }

    let parser = TestParser::new("imsu:");
    let result = parser.parse_flags();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_flags_duplicate_flags() {
    struct TestParser {
        pattern: String,
        current_index: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                current_index: 0,
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.current_index).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.current_index < self.pattern.len() - 1 {
                self.current_index += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span {
                start: Position { offset: 0, line: 1, column: 1 },
                end: Position { offset: self.current_index, line: 1, column: self.current_index + 1 },
            }
        }

        fn span_char(&self) -> Span {
            self.span()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }

        fn parse_flag(&self) -> Result<ast::Flag> {
            match self.char() {
                'i' => Ok(ast::Flag::CaseInsensitive),
                'm' => Ok(ast::Flag::MultiLine),
                's' => Ok(ast::Flag::DotMatchesNewLine),
                'U' => Ok(ast::Flag::SwapGreed),
                'u' => Ok(ast::Flag::Unicode),
                'x' => Ok(ast::Flag::IgnoreWhitespace),
                _ => Err(self.error(self.span_char(), ast::ErrorKind::FlagUnrecognized)),
            }
        }

        fn pos(&self) -> Position {
            Position { offset: self.current_index, line: 1, column: self.current_index + 1 }
        }

        fn parse_flags(&self) -> Result<ast::Flags> {
            let mut flags = ast::Flags {
                span: self.span(),
                items: vec![],
            };
            while self.char() != ':' && self.char() != ')' {
                let item = ast::FlagsItem {
                    span: self.span_char(),
                    kind: ast::FlagsItemKind::Flag(self.parse_flag()?),
                };
                if let Some(i) = flags.add_item(item) {
                    return Err(self.error(self.span_char(), ast::ErrorKind::FlagDuplicate {
                        original: flags.items[i].span,
                    }));
                }
                if !self.bump() {
                    return Err(self.error(self.span(), ast::ErrorKind::FlagUnexpectedEof));
                }
            }
            flags.span.end = self.pos();
            Ok(flags)
        }
    }

    let parser = TestParser::new("ii:");
    let _ = parser.parse_flags();
}

#[test]
#[should_panic]
fn test_parse_flags_dangling_negation() {
    struct TestParser {
        pattern: String,
        current_index: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                current_index: 0,
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.current_index).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.current_index < self.pattern.len() - 1 {
                self.current_index += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span {
                start: Position { offset: 0, line: 1, column: 1 },
                end: Position { offset: self.current_index, line: 1, column: self.current_index + 1 },
            }
        }

        fn span_char(&self) -> Span {
            self.span()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }

        fn parse_flag(&self) -> Result<ast::Flag> {
            match self.char() {
                'i' => Ok(ast::Flag::CaseInsensitive),
                'm' => Ok(ast::Flag::MultiLine),
                's' => Ok(ast::Flag::DotMatchesNewLine),
                'U' => Ok(ast::Flag::SwapGreed),
                'u' => Ok(ast::Flag::Unicode),
                'x' => Ok(ast::Flag::IgnoreWhitespace),
                _ => Err(self.error(self.span_char(), ast::ErrorKind::FlagUnrecognized)),
            }
        }

        fn pos(&self) -> Position {
            Position { offset: self.current_index, line: 1, column: self.current_index + 1 }
        }

        fn parse_flags(&self) -> Result<ast::Flags> {
            let mut flags = ast::Flags {
                span: self.span(),
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
                        return Err(self.error(self.span_char(), ast::ErrorKind::FlagDuplicate {
                            original: flags.items[i].span,
                        }));
                    }
                } else {
                    let item = ast::FlagsItem {
                        span: self.span_char(),
                        kind: ast::FlagsItemKind::Flag(self.parse_flag()?),
                    };
                    if let Some(i) = flags.add_item(item) {
                        return Err(self.error(self.span_char(), ast::ErrorKind::FlagDuplicate {
                            original: flags.items[i].span,
                        }));
                    }
                }
                if !self.bump() {
                    return Err(self.error(self.span(), ast::ErrorKind::FlagUnexpectedEof));
                }
            }
            if last_was_negation.is_some() {
                return Err(self.error(last_was_negation.unwrap(), ast::ErrorKind::FlagDanglingNegation));
            }
            flags.span.end = self.pos();
            Ok(flags)
        }
    }

    let parser = TestParser::new("-:");
    let _ = parser.parse_flags();
}

