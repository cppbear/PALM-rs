// Answer 0

#[test]
fn test_parse_decimal_valid_number() {
    struct Parser {
        input: &'static str,
        position: usize,
    }

    impl Parser {
        fn new(input: &'static str) -> Self {
            Parser { input, position: 0 }
        }

        fn char(&self) -> char {
            self.input[self.position..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += self.char().len_utf8();
            }
        }

        fn bump_and_bump_space(&mut self) {
            self.bump();
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<u32> {
            Err("error".into()) // Simplified error return for testing
        }

        fn parser(&self) -> &Self {
            self
        }

        fn parse_decimal(&mut self) -> Result<u32> {
            let mut scratch = String::new();

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
            let digits = &scratch;
            if digits.is_empty() {
                return Err(self.error(span, ast::ErrorKind::DecimalEmpty));
            }
            match u32::from_str_radix(digits, 10).ok() {
                Some(n) => Ok(n),
                None => Err(self.error(span, ast::ErrorKind::DecimalInvalid)),
            }
        }
    }

    struct Span {
        start: usize,
        end: usize,
    }

    impl Span {
        fn new(start: usize, end: usize) -> Self {
            Span { start, end }
        }
    }

    mod ast {
        #[derive(Debug)]
        pub enum ErrorKind {
            DecimalEmpty,
            DecimalInvalid,
        }
    }

    // Test for valid input
    let mut parser = Parser::new("   12345   ");
    let result = parser.parse_decimal().unwrap();
    assert_eq!(result, 12345);
}

#[test]
fn test_parse_decimal_empty() {
    struct Parser {
        input: &'static str,
        position: usize,
    }

    impl Parser {
        fn new(input: &'static str) -> Self {
            Parser { input, position: 0 }
        }

        fn char(&self) -> char {
            self.input[self.position..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += self.char().len_utf8();
            }
        }

        fn bump_and_bump_space(&mut self) {
            self.bump();
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<u32> {
            Err("error".into()) // Simplified error return for testing
        }

        fn parser(&self) -> &Self {
            self
        }

        fn parse_decimal(&mut self) -> Result<u32> {
            let mut scratch = String::new();

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
            let digits = &scratch;
            if digits.is_empty() {
                return Err(self.error(span, ast::ErrorKind::DecimalEmpty));
            }
            match u32::from_str_radix(digits, 10).ok() {
                Some(n) => Ok(n),
                None => Err(self.error(span, ast::ErrorKind::DecimalInvalid)),
            }
        }
    }

    struct Span {
        start: usize,
        end: usize,
    }

    impl Span {
        fn new(start: usize, end: usize) -> Self {
            Span { start, end }
        }
    }

    mod ast {
        #[derive(Debug)]
        pub enum ErrorKind {
            DecimalEmpty,
            DecimalInvalid,
        }
    }

    // Test for empty input
    let mut parser = Parser::new("   ");
    let result = parser.parse_decimal();
    assert!(result.is_err());
}

#[test]
fn test_parse_decimal_invalid() {
    struct Parser {
        input: &'static str,
        position: usize,
    }

    impl Parser {
        fn new(input: &'static str) -> Self {
            Parser { input, position: 0 }
        }

        fn char(&self) -> char {
            self.input[self.position..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += self.char().len_utf8();
            }
        }

        fn bump_and_bump_space(&mut self) {
            self.bump();
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<u32> {
            Err("error".into()) // Simplified error return for testing
        }

        fn parser(&self) -> &Self {
            self
        }

        fn parse_decimal(&mut self) -> Result<u32> {
            let mut scratch = String::new();

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
            let digits = &scratch;
            if digits.is_empty() {
                return Err(self.error(span, ast::ErrorKind::DecimalEmpty));
            }
            match u32::from_str_radix(digits, 10).ok() {
                Some(n) => Ok(n),
                None => Err(self.error(span, ast::ErrorKind::DecimalInvalid)),
            }
        }
    }

    struct Span {
        start: usize,
        end: usize,
    }

    impl Span {
        fn new(start: usize, end: usize) -> Self {
            Span { start, end }
        }
    }

    mod ast {
        #[derive(Debug)]
        pub enum ErrorKind {
            DecimalEmpty,
            DecimalInvalid,
        }
    }

    // Test for invalid input
    let mut parser = Parser::new("abc");
    let result = parser.parse_decimal();
    assert!(result.is_err());
}

