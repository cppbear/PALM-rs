// Answer 0

#[test]
fn test_parse_unicode_class_positive_case_with_one_letter() {
    struct Parser {
        input: Vec<char>,
        pos: usize,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Self { input: input.chars().collect(), pos: 0 }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                self.pos += 1; // bumping the character
                return true;
            }
            false
        }

        fn span_char(&self) -> (usize, usize) {
            (self.pos, self.pos + 1)
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: (usize, usize), _kind: &str) -> &str {
            "Error"
        }
    }

    let mut parser = Parser::new("p x");
    let result = parse_unicode_class(&mut parser);
    assert!(result.is_ok());
}

#[test]
fn test_parse_unicode_class_negative_case_with_one_letter() {
    struct Parser {
        input: Vec<char>,
        pos: usize,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Self { input: input.chars().collect(), pos: 0 }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                self.pos += 1; // bumping the character
                return true;
            }
            false
        }

        fn span_char(&self) -> (usize, usize) {
            (self.pos, self.pos + 1)
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: (usize, usize), _kind: &str) -> &str {
            "Error"
        }
    }

    let mut parser = Parser::new("p p");
    let result = parse_unicode_class(&mut parser);
    assert!(result.is_ok());
}

#[test]
fn test_parse_unicode_class_eof_error() {
    struct Parser {
        input: Vec<char>,
        pos: usize,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Self { input: input.chars().collect(), pos: 0 }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                self.pos += 1; // bumping the character
                return true;
            }
            false
        }

        fn span_char(&self) -> (usize, usize) {
            (self.pos, self.pos + 1)
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: (usize, usize), _kind: &str) -> &str {
            "Error"
        }
    }

    let mut parser = Parser::new("p {");
    let result = parse_unicode_class(&mut parser);
    assert!(result.is_err());
}

