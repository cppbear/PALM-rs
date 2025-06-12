// Answer 0

#[test]
fn test_parse_decimal_empty() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn parse_decimal(&mut self) -> Result<u32, String> {
            let mut scratch = Vec::new();
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
            let start = self.pos();
            while !self.is_eof() && '0' <= self.char() && self.char() <= '9' {
                scratch.push(self.char());
                self.bump();
            }
            let digits = scratch.iter().collect::<String>();
            if digits.is_empty() {
                return Err("DecimalEmpty".to_string());
            }
            match u32::from_str_radix(&digits, 10).ok() {
                Some(n) => Ok(n),
                None => Err("DecimalInvalid".to_string()),
            }
        }
    }

    let mut parser = MockParser::new("  \n  ");
    let result = parser.parse_decimal();
    assert_eq!(result, Err("DecimalEmpty".to_string()));
}

#[test]
fn test_parse_decimal_valid() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn parse_decimal(&mut self) -> Result<u32, String> {
            let mut scratch = Vec::new();
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
            let start = self.pos();
            while !self.is_eof() && '0' <= self.char() && self.char() <= '9' {
                scratch.push(self.char());
                self.bump();
            }
            let digits = scratch.iter().collect::<String>();
            if digits.is_empty() {
                return Err("DecimalEmpty".to_string());
            }
            match u32::from_str_radix(&digits, 10).ok() {
                Some(n) => Ok(n),
                None => Err("DecimalInvalid".to_string()),
            }
        }
    }

    let mut parser = MockParser::new("  123  ");
    let result = parser.parse_decimal();
    assert_eq!(result, Ok(123));
}

#[test]
fn test_parse_decimal_invalid() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn parse_decimal(&mut self) -> Result<u32, String> {
            let mut scratch = Vec::new();
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
            let start = self.pos();
            while !self.is_eof() && '0' <= self.char() && self.char() <= '9' {
                scratch.push(self.char());
                self.bump();
            }
            let digits = scratch.iter().collect::<String>();
            if digits.is_empty() {
                return Err("DecimalEmpty".to_string());
            }
            match u32::from_str_radix(&digits, 10).ok() {
                Some(n) => Ok(n),
                None => Err("DecimalInvalid".to_string()),
            }
        }
    }

    let mut parser = MockParser::new("  123abc  ");
    let result = parser.parse_decimal();
    assert_eq!(result, Err("DecimalInvalid".to_string()));
}

#[test]
fn test_parse_decimal_no_digits() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn parse_decimal(&mut self) -> Result<u32, String> {
            let mut scratch = Vec::new();
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
            let start = self.pos();
            while !self.is_eof() && '0' <= self.char() && self.char() <= '9' {
                scratch.push(self.char());
                self.bump();
            }
            let digits = scratch.iter().collect::<String>();
            if digits.is_empty() {
                return Err("DecimalEmpty".to_string());
            }
            match u32::from_str_radix(&digits, 10).ok() {
                Some(n) => Ok(n),
                None => Err("DecimalInvalid".to_string()),
            }
        }
    }

    let mut parser = MockParser::new("   abc   ");
    let result = parser.parse_decimal();
    assert_eq!(result, Err("DecimalEmpty".to_string()));
}

