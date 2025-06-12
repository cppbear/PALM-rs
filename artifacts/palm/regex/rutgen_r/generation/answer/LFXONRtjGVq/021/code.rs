// Answer 0

#[test]
fn test_parse_counted_repetition_invalid_char() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self { input: input.chars().collect(), pos: 0 }
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

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1; // Assume there's a space after the '{'
            true
        }

        fn parse_decimal(&mut self) -> Result<usize, ()> {
            // Simulate a valid number parse
            Ok(3)
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> () {
            // Simulate error handling
        }
    }

    let mut concat = ast::Concat { asts: vec![ast::Ast::Dummy] }; // Assuming Ast::Dummy exists
    let mut parser = MockParser::new("}"); // Invalid character before expecting '}'

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_missing_repetition_count() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self { input: input.chars().collect(), pos: 0 }
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

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1; // Assume there's a space after the '{'
            true
        }

        fn parse_decimal(&mut self) -> Result<usize, ()> {
            // Valid number
            Ok(3)
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> () {
            // Simulate error handling
        }
    }

    let mut concat = ast::Concat { asts: vec![ast::Ast::Dummy] };
    let mut parser = MockParser::new("{3,"); // Missing '}'

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_invalid_repetition_count() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self { input: input.chars().collect(), pos: 0 }
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

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1; // Assume there's a space after the '{'
            true
        }

        fn parse_decimal(&mut self) -> Result<usize, ()> {
            // Simulate an invalid number parse
            Err(())
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> () {
            // Simulate error handling
        }
    }

    let mut concat = ast::Concat { asts: vec![ast::Ast::Dummy] };
    let mut parser = MockParser::new("{3,");

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

