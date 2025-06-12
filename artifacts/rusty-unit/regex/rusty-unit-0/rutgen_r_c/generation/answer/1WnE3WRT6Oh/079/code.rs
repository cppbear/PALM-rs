// Answer 0

#[test]
fn test_parse_decimal_success() {
    struct TestParser {
        pos: Position,
        input: String,
        index: usize,
        scratch: String,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                input: input.to_string(),
                index: 0,
                scratch: String::new(),
            }
        }

        fn bump(&mut self) {
            self.index += 1;
            self.pos.offset += 1;
        }

        fn char(&self) -> char {
            self.input[self.index..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn reset_scratch(&mut self) {
            self.scratch.clear();
        }

        fn parse_decimal(&mut self) -> Result<u32> {
            self.reset_scratch();

            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
            let start = self.pos;

            while !self.is_eof() && ('0'..='9').contains(&self.char()) {
                self.scratch.push(self.char());
                self.bump();
            }
            
            let span = Span::new(start, self.pos);
            if self.scratch.is_empty() {
                return Err(ast::Error {
                    kind: ast::ErrorKind::DecimalEmpty,
                    pattern: self.input.clone(),
                    span,
                });
            }

            u32::from_str_radix(&self.scratch).map_err(|_| ast::Error {
                kind: ast::ErrorKind::DecimalInvalid,
                pattern: self.input.clone(),
                span,
            })
        }
    }

    let mut parser = TestParser::new("123");
    assert_eq!(parser.parse_decimal(), Ok(123));

    parser = TestParser::new(" \n 456 \t ");
    assert_eq!(parser.parse_decimal(), Ok(456));

    parser = TestParser::new("     ");
    assert_eq!(parser.parse_decimal(), Err(ast::Error {
        kind: ast::ErrorKind::DecimalEmpty,
        pattern: parser.input.clone(),
        span: Span::new(parser.pos, parser.pos),
    }));
}

#[test]
fn test_parse_decimal_empty_case() {
    struct TestParser {
        pos: Position,
        input: String,
        index: usize,
        scratch: String,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                input: input.to_string(),
                index: 0,
                scratch: String::new(),
            }
        }

        fn bump(&mut self) {
            self.index += 1;
            self.pos.offset += 1;
        }

        fn char(&self) -> char {
            self.input[self.index..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn reset_scratch(&mut self) {
            self.scratch.clear();
        }

        fn parse_decimal(&mut self) -> Result<u32> {
            self.reset_scratch();

            if self.is_eof() {
                return Err(ast::Error {
                    kind: ast::ErrorKind::DecimalEmpty,
                    pattern: self.input.clone(),
                    span: Span::new(self.pos, self.pos),
                });
            }
            
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
            let start = self.pos;

            while !self.is_eof() && ('0'..='9').contains(&self.char()) {
                self.scratch.push(self.char());
                self.bump();
            }
            
            let span = Span::new(start, self.pos);
            if self.scratch.is_empty() {
                return Err(ast::Error {
                    kind: ast::ErrorKind::DecimalEmpty,
                    pattern: self.input.clone(),
                    span,
                });
            }

            u32::from_str_radix(&self.scratch).map_err(|_| ast::Error {
                kind: ast::ErrorKind::DecimalInvalid,
                pattern: self.input.clone(),
                span,
            })
        }
    }

    let mut parser = TestParser::new("");
    assert_eq!(parser.parse_decimal(), Err(ast::Error {
        kind: ast::ErrorKind::DecimalEmpty,
        pattern: parser.input.clone(),
        span: Span::new(parser.pos, parser.pos),
    }));
}

