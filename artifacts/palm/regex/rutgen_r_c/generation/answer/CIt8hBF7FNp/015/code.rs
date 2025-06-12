// Answer 0

#[test]
fn test_parse_set_class_open_with_negation_and_literal() {
    struct TestParser {
        input: Vec<char>,
        pos: Position,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.chars().collect(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos.offset < self.input.len() {
                self.pos.offset += 1;
                self.pos.column += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.pos.offset]
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span(&self) -> Span {
            Span::new(self.pos(), self.pos()) // simplistic implementation
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos()) // simplistic implementation
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new(kind, "Test Error".to_string(), span) // simplistic error creation
        }
    }

    let mut parser = TestParser::new("[^-]");

    let result = parser.parse_set_class_open();

    assert!(result.is_ok());
    let (set, union) = result.unwrap();

    assert_eq!(set.negated, true);
    assert_eq!(union.items.len(), 1);
    match &union.items[0] {
        ast::ClassSetItem::Literal(literal) => {
            assert_eq!(literal.kind, ast::LiteralKind::Verbatim);
            assert_eq!(literal.c, '-');
        }
        _ => panic!("Expected a literal item in union"),
    }
}

#[test]
fn test_parse_set_class_open_with_empty_class() {
    struct TestParser {
        input: Vec<char>,
        pos: Position,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.chars().collect(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos.offset < self.input.len() {
                self.pos.offset += 1;
                self.pos.column += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.pos.offset]
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span(&self) -> Span {
            Span::new(self.pos(), self.pos()) // simplistic implementation
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos()) // simplistic implementation
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new(kind, "Test Error".to_string(), span) // simplistic error creation
        }
    }

    let mut parser = TestParser::new("[]");

    let result = parser.parse_set_class_open();

    assert!(result.is_ok());
    let (set, union) = result.unwrap();

    assert_eq!(set.negated, false);
    assert_eq!(union.items.len(), 1);
    match &union.items[0] {
        ast::ClassSetItem::Literal(literal) => {
            assert_eq!(literal.kind, ast::LiteralKind::Verbatim);
            assert_eq!(literal.c, ']');
        }
        _ => panic!("Expected a literal item in union"),
    }
}

