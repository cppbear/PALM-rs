// Answer 0

#[test]
fn test_parse_set_class_range_valid_case() {
    struct DummyParser {
        cursor: usize,
        pattern: String,
    }

    impl DummyParser {
        fn new(pattern: &str) -> Self {
            Self { cursor: 0, pattern: pattern.to_string() }
        }

        fn bump(&mut self) {
            self.cursor += 1;
        }
        
        fn char(&self) -> char {
            self.pattern.chars().nth(self.cursor).unwrap()
        }

        fn peek_space(&self) -> Option<char> {
            if self.cursor + 1 < self.pattern.len() {
                Some(self.pattern.chars().nth(self.cursor + 1).unwrap())
            } else {
                None
            }
        }

        fn is_eof(&self) -> bool {
            self.cursor >= self.pattern.len()
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump();
            self.bump();
            true
        }
        
        fn parse_set_class_item(&self) -> Result<Primitive> {
            let lit = Literal {
                span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
                kind: LiteralKind::Verbatim,
                c: self.char(),
            };
            Ok(Primitive::Literal(lit))
        }

        fn unclosed_class_error(&self) -> Error {
            Error {
                kind: ErrorKind::ClassUnclosed,
                pattern: self.pattern.clone(),
                span: Span::new(Position { offset: self.cursor, line: 1, column: 1 }, Position { offset: self.cursor + 1, line: 1, column: 2 }),
            }
        }
    }

    let parser = DummyParser::new("a-b");
    assert_eq!(parser.parse_set_class_range().unwrap_err().kind, ErrorKind::ClassUnclosed);
}

