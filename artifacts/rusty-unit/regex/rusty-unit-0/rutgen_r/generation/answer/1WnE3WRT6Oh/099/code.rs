// Answer 0

#[test]
fn test_parse_decimal_invalid_digit() {
    struct ParserMock {
        input: &'static str,
        position: usize,
    }

    impl ParserMock {
        fn new(input: &'static str) -> Self {
            Self { input, position: 0 }
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.position..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += self.char().len_utf8();
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump_and_bump_space(&mut self) {
            self.bump();
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<u32, ast::Error> {
            Err(ast::Error::new(kind, span))
        }

        fn scratch(&self) -> std::cell::RefCell<Vec<char>> {
            std::cell::RefCell::new(Vec::new())
        }
    }

    impl ParserMock {
        fn parse_decimal(&mut self) -> Result<u32, ast::Error> {
            let mut scratch = self.scratch().borrow_mut();
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
                return Err(self.error(span, ast::ErrorKind::DecimalEmpty));
            }
            match u32::from_str_radix(digits, 10).ok() {
                Some(n) => Ok(n),
                None => Err(self.error(span, ast::ErrorKind::DecimalInvalid)),
            }
        }
    }

    let mut parser = ParserMock::new("123abc");
    let result = parser.parse_decimal();
    assert!(result.is_err());
}

#[test]
fn test_parse_decimal_empty() {
    struct ParserMock {
        input: &'static str,
        position: usize,
    }

    impl ParserMock {
        fn new(input: &'static str) -> Self {
            Self { input, position: 0 }
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.position..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += self.char().len_utf8();
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump_and_bump_space(&mut self) {
            self.bump();
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<u32, ast::Error> {
            Err(ast::Error::new(kind, span))
        }

        fn scratch(&self) -> std::cell::RefCell<Vec<char>> {
            std::cell::RefCell::new(Vec::new())
        }
    }

    impl ParserMock {
        fn parse_decimal(&mut self) -> Result<u32, ast::Error> {
            let mut scratch = self.scratch().borrow_mut();
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
                return Err(self.error(span, ast::ErrorKind::DecimalEmpty));
            }
            match u32::from_str_radix(digits, 10).ok() {
                Some(n) => Ok(n),
                None => Err(self.error(span, ast::ErrorKind::DecimalInvalid)),
            }
        }
    }

    let mut parser = ParserMock::new("   "); // Only whitespace
    let result = parser.parse_decimal();
    assert!(result.is_err());
}

