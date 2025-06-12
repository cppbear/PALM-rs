// Answer 0

#[test]
fn test_parse_counted_repetition_valid_exact() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1; 
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.char().is_whitespace() {
                self.bump();
            }
            if self.char() != '\0' {
                self.bump();
                return true;
            }
            false
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn parse_decimal(&mut self) -> Result<usize, ()> {
            let mut num = 0;
            while self.char().is_digit(10) {
                num = num * 10 + self.char().to_digit(10).unwrap() as usize;
                self.bump();
            }
            Ok(num)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> () {
            panic!("Parsing error");
        }
    }

    let mut parser = MockParser::new("{2}");
    let concat = ast::Concat { asts: vec![] };
    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_ok());
}

#[test]
fn test_parse_counted_repetition_valid_bounded() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1; 
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.char().is_whitespace() {
                self.bump();
            }
            if self.char() != '\0' {
                self.bump();
                return true;
            }
            false
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn parse_decimal(&mut self) -> Result<usize, ()> {
            let mut num = 0;
            while self.char().is_digit(10) {
                num = num * 10 + self.char().to_digit(10).unwrap() as usize;
                self.bump();
            }
            Ok(num)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> () {
            panic!("Parsing error");
        }
    }

    let mut parser = MockParser::new("{2,4}");
    let concat = ast::Concat { asts: vec![] };
    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_ok());
}

#[test]
fn test_parse_counted_repetition_invalid_unclosed() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1; 
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.char().is_whitespace() {
                self.bump();
            }
            if self.char() != '\0' {
                self.bump();
                return true;
            }
            false
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn parse_decimal(&mut self) -> Result<usize, ()> {
            let mut num = 0;
            while self.char().is_digit(10) {
                num = num * 10 + self.char().to_digit(10).unwrap() as usize;
                self.bump();
            }
            Ok(num)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> () {
            panic!("Parsing error");
        }
    }

    let mut parser = MockParser::new("{2,");
    let concat = ast::Concat { asts: vec![] };
    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

