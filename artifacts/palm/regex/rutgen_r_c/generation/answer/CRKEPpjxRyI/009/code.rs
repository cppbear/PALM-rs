// Answer 0

#[test]
fn test_parse_set_class_range_valid_range() {
    #[derive(Clone, Debug)]
    struct MockParser {
        pattern: String,
        index: usize,
    }

    impl MockParser {
        fn new(pattern: &str) -> MockParser {
            MockParser {
                pattern: pattern.to_string(),
                index: 0,
            }
        }

        fn bump(&mut self) {
            self.index += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.index < self.pattern.len() {
                self.bump();
                return true;
            }
            false
        }

        fn is_eof(&self) -> bool {
            self.index >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.index..].chars().next().unwrap()
        }

        fn peek_space(&self) -> Option<char> {
            self.pattern[self.index..].chars().nth(1)
        }

        fn parse_set_class_item(&self) -> Result<Primitive> {
            let c = self.char();
            let span = Span::new(Position { offset: self.index, line: 1, column: 1}, Position { offset: self.index + 1, line: 1, column: 2 });
            Ok(Primitive::Literal(Literal { span, kind: ast::LiteralKind::Verbatim, c }))
        }

        fn error(&self, _span: &Span, _kind: ast::ErrorKind) -> Error {
            Error {
                kind: ast::ErrorKind::ClassRangeInvalid,
                pattern: self.pattern.clone(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
            }
        }
    }

    impl ParserI<'_, MockParser> {
        fn parse_set_class_range(&self) -> Result<ast::ClassSetItem> {
            let prim1 = self.parse_set_class_item()?;
            self.bump_space(); // Simulate space bump
            if self.is_eof() {
                return Err(self.unclosed_class_error());
            }
            // Expecting `-` to indicate a range
            assert_eq!(self.char(), '-');
            self.bump_and_bump_space();
            let prim2 = self.parse_set_class_item()?;
            let range = ast::ClassSetRange {
                span: Span::new(prim1.span().start, prim2.span().end),
                start: prim1.into_class_literal(self)?,
                end: prim2.into_class_literal(self)?,
            };
            if !range.is_valid() {
                return Err(self.error(range.span, ast::ErrorKind::ClassRangeInvalid));
            }
            Ok(ast::ClassSetItem::Range(range))
        }

        fn bump_space(&mut self) {
            // Simulate bumping past potential whitespace for this mock parser
            if self.index < self.pattern.len() {
                self.bump();
            }
        }

        fn unclosed_class_error(&self) -> ast::Error {
            self.error(&(self.pattern.len().into()), ast::ErrorKind::ClassUnclosed)
        }
    }

    let parser = MockParser::new("a-b");
    let parser_instance = ParserI { parser: &parser, pattern: "a-b" };
    let result = parser_instance.parse_set_class_range();
    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_range_invalid_range() {
    #[derive(Clone, Debug)]
    struct MockParser {
        pattern: String,
        index: usize,
    }

    impl MockParser {
        fn new(pattern: &str) -> MockParser {
            MockParser {
                pattern: pattern.to_string(),
                index: 0,
            }
        }

        fn bump(&mut self) {
            self.index += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.index < self.pattern.len() {
                self.bump();
                return true;
            }
            false
        }

        fn is_eof(&self) -> bool {
            self.index >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.index..].chars().next().unwrap()
        }

        fn peek_space(&self) -> Option<char> {
            self.pattern[self.index..].chars().nth(1)
        }

        fn parse_set_class_item(&self) -> Result<Primitive> {
            let c = self.char();
            let span = Span::new(Position { offset: self.index, line: 1, column: 1 }, Position { offset: self.index + 1, line: 1, column: 2 });
            Ok(Primitive::Literal(Literal { span, kind: ast::LiteralKind::Verbatim, c }))
        }
        
        fn error(&self, _span: &Span, _kind: ast::ErrorKind) -> Error {
            Error {
                kind: ast::ErrorKind::ClassRangeInvalid,
                pattern: self.pattern.clone(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
            }
        }
    }

    impl ParserI<'_, MockParser> {
        fn parse_set_class_range(&self) -> Result<ast::ClassSetItem> {
            let prim1 = self.parse_set_class_item()?;
            self.bump_space(); // Simulate space bump
            if self.is_eof() {
                return Err(self.unclosed_class_error());
            }
            // Expecting `-` to indicate a range
            assert_eq!(self.char(), '-');
            self.bump_and_bump_space();
            let prim2 = self.parse_set_class_item()?;
            
            let range = ast::ClassSetRange {
                span: Span::new(prim1.span().start, prim2.span().end),
                start: prim1.into_class_literal(self)?,
                end: prim2.into_class_literal(self)?,
            };
            if !range.is_valid() {
                return Err(self.error(range.span, ast::ErrorKind::ClassRangeInvalid));
            }
            Ok(ast::ClassSetItem::Range(range))
        }

        fn bump_space(&mut self) {
            // Simulate bumping past potential whitespace for this mock parser
            if self.index < self.pattern.len() {
                self.bump();
            }
        }

        fn unclosed_class_error(&self) -> ast::Error {
            self.error(&(self.pattern.len().into()), ast::ErrorKind::ClassUnclosed)
        }
    }

    let parser = MockParser::new("b-a"); // Invalid range b-a, since b comes after a
    let parser_instance = ParserI { parser: &parser, pattern: "b-a" };
    let result = parser_instance.parse_set_class_range();
    assert!(result.is_err());
}

