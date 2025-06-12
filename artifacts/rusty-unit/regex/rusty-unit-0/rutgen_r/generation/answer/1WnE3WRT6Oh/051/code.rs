// Answer 0

#[test]
fn test_parse_decimal_empty() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.pos]
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.pos += 1;
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<u32> {
            Err(kind) // Assume Result type is simplified to return ErrorKind directly.
        }

        fn bump_and_bump_space(&mut self) {
            self.bump();
        }

        fn scratch(&self) -> RefMut<Vec<char>> {
            // simulating borrowing a mutable scratch for the example
            RefMut::map(Cell::new(Vec::new()), |v| v)
        }
    }

    impl TestParser {
        fn parse_decimal(&mut self) -> Result<u32> {
            let mut scratch = self.scratch();
            scratch.clear();

            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }

            let start = self.pos();
            while !self.is_eof() && '0' <= self.char() && self.char() <= '9' {
                scratch.push(self.char());
                self.bump_and_bump_space();
            }

            let span = Span::new(start, self.pos());
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump_and_bump_space();
            }

            let digits = scratch.as_str();
            if digits.is_empty() {
                return self.error(span, ast::ErrorKind::DecimalEmpty);
            }

            match u32::from_str_radix(digits, 10).ok() {
                Some(n) => Ok(n),
                None => self.error(span, ast::ErrorKind::DecimalInvalid),
            }
        }
    }

    // Using empty input should trigger `DecimalEmpty` error
    let mut parser = TestParser { input: Vec::new(), pos: 0 };
    let result = parser.parse_decimal();
    assert_eq!(result, Err(ast::ErrorKind::DecimalEmpty));
}

#[test]
fn test_parse_decimal_invalid() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.pos]
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.pos += 1;
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<u32> {
            Err(kind) // Assume Result type is simplified to return ErrorKind directly.
        }

        fn bump_and_bump_space(&mut self) {
            self.bump();
        }
    }

    impl TestParser {
        fn parse_decimal(&mut self) -> Result<u32> {
            let mut scratch = self.scratch();
            scratch.clear();

            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }

            let start = self.pos();
            while !self.is_eof() && '0' <= self.char() && self.char() <= '9' {
                scratch.push(self.char());
                self.bump_and_bump_space();
            }

            let span = Span::new(start, self.pos());
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump_and_bump_space();
            }

            let digits = scratch.as_str();
            if digits.is_empty() {
                return self.error(span, ast::ErrorKind::DecimalEmpty);
            }

            match u32::from_str_radix(digits, 10).ok() {
                Some(n) => Ok(n),
                None => self.error(span, ast::ErrorKind::DecimalInvalid),
            }
        }
    }

    // Using invalid digits should trigger `DecimalInvalid` error
    let mut parser = TestParser { input: vec!['a'], pos: 0 };
    let result = parser.parse_decimal();
    assert_eq!(result, Err(ast::ErrorKind::DecimalInvalid));
}

