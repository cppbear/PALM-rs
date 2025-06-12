// Answer 0

#[test]
fn test_parse_set_class_open_with_empty_class() {
    struct MockParser {
        input: &'static str,
        pos: Position,
    }
    
    impl MockParser {
        fn new(input: &'static str) -> Self {
            MockParser {
                input,
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos.offset += 1;
            // Simulate bumping by skipping whitespace if any
            while self.char().is_whitespace() {
                self.pos.offset += 1;
            }
            true
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            self.span()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from(self.input), span }
        }
    }

    let mut parser = MockParser::new("[^]");
    let result = parser.parse_set_class_open();
    let expected_union = ast::ClassSetUnion {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        items: vec![ast::ClassSetItem::Literal(ast::Literal {
            span: Span::new(Position { offset: 1, line: 1, column: 2 }, Position { offset: 1, line: 1, column: 2 }),
            kind: ast::LiteralKind::Verbatim,
            c: ']',
        })],
    };

    assert_eq!(result, Ok((
        ast::ClassBracketed {
            span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 3, line: 1, column: 4 }),
            negated: true,
            kind: ast::ClassSet::union(expected_union.clone()),
        },
        expected_union,
    )));
}

#[test]
#[should_panic]
fn test_parse_set_class_open_with_unclosed_class() {
    struct MockParser {
        input: &'static str,
        pos: Position,
    }
    
    impl MockParser {
        fn new(input: &'static str) -> Self {
            MockParser {
                input,
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos.offset += 1;
            // Simulate bumping by skipping whitespace if any
            while self.char().is_whitespace() {
                self.pos.offset += 1;
            }
            false // Simulating failure when bumping
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            self.span()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from(self.input), span }
        }
    }

    let mut parser = MockParser::new("[^");
    parser.parse_set_class_open();
}

