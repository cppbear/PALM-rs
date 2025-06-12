// Answer 0

#[test]
fn test_parse_counted_repetition_valid_counts() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.bump();
                return true;
            }
            false
        }

        fn parse_decimal(&mut self) -> Result<usize, String> {
            let start = self.pos;
            while self.pos < self.input.len() && self.input[self.pos].is_digit(10) {
                self.bump();
            }
            let num_str: String = self.input[start..self.pos].iter().collect();
            num_str.parse::<usize>().map_err(|_| "Invalid number".to_string())
        }

        fn error(&self, span: usize, kind: &str) -> String {
            format!("Error at position {}: {}", span, kind)
        }
    }

    let parser = &mut TestParser {
        input: vec!['{', '2', ',', '5', '}', ' '],
        pos: 0,
    };

    let result = parser.parse_counted_repetition(ast::Concat { asts: vec![ast::Ast::Base] });
    assert!(result.is_ok());
}

#[test]
fn test_parse_counted_repetition_single_count() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.bump();
                return true;
            }
            false
        }

        fn parse_decimal(&mut self) -> Result<usize, String> {
            let start = self.pos;
            while self.pos < self.input.len() && self.input[self.pos].is_digit(10) {
                self.bump();
            }
            let num_str: String = self.input[start..self.pos].iter().collect();
            num_str.parse::<usize>().map_err(|_| "Invalid number".to_string())
        }

        fn error(&self, span: usize, kind: &str) -> String {
            format!("Error at position {}: {}", span, kind)
        }
    }

    let parser = &mut TestParser {
        input: vec!['{', '3', '}', ' '],
        pos: 0,
    };

    let result = parser.parse_counted_repetition(ast::Concat { asts: vec![ast::Ast::Base] });
    assert!(result.is_ok());
}

#[should_panic(expected = "Error at position 2: RepetitionCountUnclosed")]
#[test]
fn test_parse_counted_repetition_unclosed() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.bump();
                return true;
            }
            false
        }

        fn parse_decimal(&mut self) -> Result<usize, String> {
            let start = self.pos;
            while self.pos < self.input.len() && self.input[self.pos].is_digit(10) {
                self.bump();
            }
            let num_str: String = self.input[start..self.pos].iter().collect();
            num_str.parse::<usize>().map_err(|_| "Invalid number".to_string())
        }

        fn error(&self, span: usize, kind: &str) -> String {
            format!("Error at position {}: {}", span, kind)
        }
    }

    let parser = &mut TestParser {
        input: vec!['{', '3', ',', '4'], // Missing closing '}' 
        pos: 0,
    };

    parser.parse_counted_repetition(ast::Concat { asts: vec![ast::Ast::Base] }).unwrap();
}

#[should_panic(expected = "Error at position 7: RepetitionCountInvalid")]
#[test]
fn test_parse_counted_repetition_invalid_count() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.bump();
                return true;
            }
            false
        }

        fn parse_decimal(&mut self) -> Result<usize, String> {
            let start = self.pos;
            while self.pos < self.input.len() && self.input[self.pos].is_digit(10) {
                self.bump();
            }
            let num_str: String = self.input[start..self.pos].iter().collect();
            num_str.parse::<usize>().map_err(|_| "Invalid number".to_string())
        }

        fn error(&self, span: usize, kind: &str) -> String {
            format!("Error at position {}: {}", span, kind)
        }
    }

    let parser = &mut TestParser {
        input: vec!['{', '0', ',', '3', '}', ' '], // Invalid zero in repetition
        pos: 0,
    };

    parser.parse_counted_repetition(ast::Concat { asts: vec![ast::Ast::Base] }).unwrap();
}

