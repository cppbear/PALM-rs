// Answer 0

#[test]
fn test_parse_counted_repetition_invalid_self_char() {
    struct DummyParser {
        pos: usize,
        chars: Vec<char>,
    }

    impl DummyParser {
        fn new(chars: Vec<char>) -> Self {
            Self { pos: 0, chars }
        }

        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump(); // Simulate bumping
            true // Simulate that space bump is successful
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }

        fn parse_decimal(&self) -> Result<usize, ()> {
            Err(()) // Simulated error case
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> () {
            panic!(); // Trigger panic
        }
    }

    let parser = DummyParser::new(vec!['a']); // Starting char is not '{'
    let concat = ast::Concat { asts: vec![ast::Ast::Empty] };
    
    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_empty_concat() {
    struct DummyParser {
        pos: usize,
        chars: Vec<char>,
    }

    impl DummyParser {
        fn new(chars: Vec<char>) -> Self {
            Self { pos: 0, chars }
        }

        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump(); // Simulate bumping
            true // Simulate that space bump is successful
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }

        fn parse_decimal(&self) -> Result<usize, ()> {
            Ok(1) // Simulated success case
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> () {
            panic!(); // Trigger panic
        }
    }

    let parser = DummyParser::new(vec!['{', '1', '}']); // Valid input following '{'
    let concat = ast::Concat { asts: vec![] }; // Empty concat

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_missing_ending_brace() {
    struct DummyParser {
        pos: usize,
        chars: Vec<char>,
    }

    impl DummyParser {
        fn new(chars: Vec<char>) -> Self {
            Self { pos: 0, chars }
        }

        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump(); // Simulate bumping
            true // Simulate that space bump is successful
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len() // Simulating EOF
        }

        fn parse_decimal(&self) -> Result<usize, ()> {
            Ok(1) // Simulated success case
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> () {
            panic!(); // Trigger panic
        }
    }

    let parser = DummyParser::new(vec!['{', '1', ',', '1']); // Missing closing '}'
    let concat = ast::Concat { asts: vec![ast::Ast::Empty] };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_invalid_decimal() {
    struct DummyParser {
        pos: usize,
        chars: Vec<char>,
    }

    impl DummyParser {
        fn new(chars: Vec<char>) -> Self {
            Self { pos: 0, chars }
        }

        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump(); // Simulate bumping
            true // Simulate that space bump is successful
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }

        fn parse_decimal(&self) -> Result<usize, ()> {
            Err(()) // Simulated error
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> () {
            panic!(); // Trigger panic
        }
    }

    let parser = DummyParser::new(vec!['{', '1', ',', 'a', '}']); // Invalid decimal character
    let concat = ast::Concat { asts: vec![ast::Ast::Empty] };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

