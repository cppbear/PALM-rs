// Answer 0

fn test_parse_set_class_range_valid_literal_range() {
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

        fn bump_space(&mut self) {
            // Simulated space bumping logic for the sake of the test.
            self.pos.offset += 1;
            self.pos.column += 1;
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.offset >= self.input.len()
        }

        fn peek_space(&self) -> Option<char> {
            self.input.chars().nth(self.pos.offset + 1)
        }

        fn parse_set_class_item(&self) -> Result<Primitive> {
            // Simulating successful parsing of literal.
            let span = Span::new(self.pos, self.pos);
            Ok(Primitive::Literal(Literal { span, kind: LiteralKind::Verbatim, c: 'a' }))
        }

        fn parse_set_class_range(&self) -> Result<ast::ClassSetItem> {
            let prim1 = self.parse_set_class_item()?;
            self.bump_space();
            if self.is_eof() {
                return Err(ast::ErrorKind::ClassEscapeInvalid.into());
            }
            // Force testing the condition to create a range
            if self.char() != '-' {
                return prim1.into_class_set_item(self);
            }
            self.bump_space();
            let prim2 = self.parse_set_class_item()?;
            let range = ast::ClassSetRange {
                span: Span::new(prim1.span().start, prim2.span().end),
                start: prim1.into_class_literal(self)?,
                end: prim2.into_class_literal(self)?,
            };
            if !range.is_valid() {
                return Err(ast::ErrorKind::ClassRangeInvalid.into());
            }
            Ok(ast::ClassSetItem::Range(range))
        }
    }

    let parser = TestParser::new("a-b");
    let result = parser.parse_set_class_range();
    assert!(result.is_ok());
}

fn test_parse_set_class_range_invalid_range() {
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

        fn bump_space(&mut self) {
            self.pos.offset += 1;
            self.pos.column += 1;
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.input.len()
        }

        fn peek_space(&self) -> Option<char> {
            self.input.chars().nth(self.pos.offset + 1)
        }

        fn parse_set_class_item(&self) -> Result<Primitive> {
            let span = Span::new(self.pos, self.pos);
            Ok(Primitive::Literal(Literal { span, kind: LiteralKind::Verbatim, c: 'b' }))
        }

        fn parse_set_class_range(&self) -> Result<ast::ClassSetItem> {
            let prim1 = self.parse_set_class_item()?;
            self.bump_space();
            if self.eof() {
                return Err(ast::ErrorKind::ClassEscapeInvalid.into());
            }
            if self.char() != '-' {
                return prim1.into_class_set_item(self);
            }
            self.bump_space();
            let prim2 = self.parse_set_class_item()?;
            let range = ast::ClassSetRange {
                span: Span::new(prim1.span().start, prim2.span().end),
                start: prim1.into_class_literal(self)?,
                end: prim2.into_class_literal(self)?,
            };
            if !range.is_valid() {
                return Err(ast::ErrorKind::ClassRangeInvalid.into());
            }
            Ok(ast::ClassSetItem::Range(range))
        }
    }

    let parser = TestParser::new("b-a");
    let result = parser.parse_set_class_range();
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind, ast::ErrorKind::ClassRangeInvalid);
}

