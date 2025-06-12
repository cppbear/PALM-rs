// Answer 0

#[test]
fn test_parse_set_class_open_with_negation_and_literals() {
    struct TestParser {
        input: String,
        pos: Position,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos.offset < self.input.len() {
                self.pos.offset += 1;
                self.pos.column += 1;
                return true;
            }
            false
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
        
        fn parse_set_class_open(&mut self) -> Result<(ast::ClassBracketed, ast::ClassSetUnion)> {
            assert_eq!(self.char(), '[');
            let start = self.pos;
            if !self.bump_and_bump_space() {
                return Err(ast::Error {
                    kind: ast::ErrorKind::ClassUnclosed,
                    pattern: self.input.clone(),
                    span: Span::new(start, self.pos),
                });
            }

            let negated = if self.char() != '^' {
                false
            } else {
                if !self.bump_and_bump_space() {
                    return Err(ast::Error {
                        kind: ast::ErrorKind::ClassUnclosed,
                        pattern: self.input.clone(),
                        span: Span::new(start, self.pos),
                    });
                }
                true
            };

            let mut union = ast::ClassSetUnion {
                span: self.span(),
                items: vec![],
            };
            while self.char() == '-' {
                union.push(ast::ClassSetItem::Literal(ast::Literal {
                    span: self.span_char(),
                    kind: ast::LiteralKind::Verbatim,
                    c: '-',
                }));
                if !self.bump_and_bump_space() {
                    return Err(ast::Error {
                        kind: ast::ErrorKind::ClassUnclosed,
                        pattern: self.input.clone(),
                        span: Span::new(start, self.pos),
                    });
                }
            }
            if union.items.is_empty() && self.char() == ']' {
                union.push(ast::ClassSetItem::Literal(ast::Literal {
                    span: self.span_char(),
                    kind: ast::LiteralKind::Verbatim,
                    c: ']',
                }));
                if !self.bump_and_bump_space() {
                    return Err(ast::Error {
                        kind: ast::ErrorKind::ClassUnclosed,
                        pattern: self.input.clone(),
                        span: Span::new(start, self.pos),
                    });
                }
            }
            let set = ast::ClassBracketed {
                span: Span::new(start, self.pos),
                negated,
                kind: ast::ClassSet::union(ast::ClassSetUnion {
                    span: Span::new(union.span.start, union.span.start),
                    items: vec![],
                }),
            };
            Ok((set, union))
        }
    }

    let mut parser = TestParser::new("[^abc-]");
    let result = parser.parse_set_class_open();
    assert!(result.is_ok());
    let (set, union) = result.unwrap();
    assert!(union.items.is_empty() == false);
}

#[test]
fn test_parse_set_class_open_with_empty_class_and_error() {
    struct TestParserWithError {
        input: String,
        pos: Position,
    }

    impl TestParserWithError {
        fn new(input: &str) -> Self {
            TestParserWithError {
                input: input.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos.offset < self.input.len() {
                self.pos.offset += 1;
                self.pos.column += 1;
                return true;
            }
            false
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
        
        fn parse_set_class_open(&mut self) -> Result<(ast::ClassBracketed, ast::ClassSetUnion)> {
            assert_eq!(self.char(), '[');
            let start = self.pos;
            if !self.bump_and_bump_space() {
                return Err(ast::Error {
                    kind: ast::ErrorKind::ClassUnclosed,
                    pattern: self.input.clone(),
                    span: Span::new(start, self.pos),
                });
            }

            let negated = if self.char() != '^' {
                false
            } else {
                if !self.bump_and_bump_space() {
                    return Err(ast::Error {
                        kind: ast::ErrorKind::ClassUnclosed,
                        pattern: self.input.clone(),
                        span: Span::new(start, self.pos),
                    });
                }
                true
            };

            let mut union = ast::ClassSetUnion {
                span: self.span(),
                items: vec![],
            };
            while self.char() == '-' {
                union.push(ast::ClassSetItem::Literal(ast::Literal {
                    span: self.span_char(),
                    kind: ast::LiteralKind::Verbatim,
                    c: '-',
                }));
                if !self.bump_and_bump_space() {
                    return Err(ast::Error {
                        kind: ast::ErrorKind::ClassUnclosed,
                        pattern: self.input.clone(),
                        span: Span::new(start, self.pos),
                    });
                }
            }
            if union.items.is_empty() && self.char() == ']' {
                union.push(ast::ClassSetItem::Literal(ast::Literal {
                    span: self.span_char(),
                    kind: ast::LiteralKind::Verbatim,
                    c: ']',
                }));
                if !self.bump_and_bump_space() {
                    return Err(ast::Error {
                        kind: ast::ErrorKind::ClassUnclosed,
                        pattern: self.input.clone(),
                        span: Span::new(start, self.pos),
                    });
                }
            }
            let set = ast::ClassBracketed {
                span: Span::new(start, self.pos),
                negated,
                kind: ast::ClassSet::union(ast::ClassSetUnion {
                    span: Span::new(union.span.start, union.span.start),
                    items: vec![],
                }),
            };
            Ok((set, union))
        }
    }

    let mut parser = TestParserWithError::new("[");
    let result = parser.parse_set_class_open();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassUnclosed);
    }
}

