// Answer 0

#[test]
fn test_parse_decimal_empty() {
    struct Parser {
        input: &'static str,
        position: usize,
    }

    impl Parser {
        fn scratch(&mut self) -> Vec<char> {
            Vec::new()
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.position..].chars().next().unwrap()
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += 1;
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

        fn error(&self, span: (usize, usize), kind: ErrorKind) -> Result<u32, ErrorKind> {
            Err(kind)
        }

        fn parse_decimal(&mut self) -> Result<u32, ErrorKind> {
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
            let span = (start, self.pos());
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump_and_bump_space();
            }
            let digits = scratch.iter().collect::<String>();
            if digits.is_empty() {
                return self.error(span, ErrorKind::DecimalEmpty);
            }
            match u32::from_str_radix(&digits, 10).ok() {
                Some(n) => Ok(n),
                None => Err(self.error(span, ErrorKind::DecimalInvalid)),
            }
        }
    }

    #[derive(Debug)]
    enum ErrorKind {
        DecimalEmpty,
        DecimalInvalid,
    }

    let mut parser = Parser {
        input: "   ",
        position: 0,
    };

    let result = parser.parse_decimal();
    assert!(result.is_err());
    match result {
        Err(ErrorKind::DecimalEmpty) => {},
        _ => panic!("Expected DecimalEmpty error"),
    }
}

