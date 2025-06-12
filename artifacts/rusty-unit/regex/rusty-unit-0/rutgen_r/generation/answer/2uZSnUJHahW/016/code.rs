// Answer 0

#[test]
fn test_parse_hex_brace_invalid_digit() {
    struct TestParser {
        position: usize,
        input: Vec<char>,
        scratch: Vec<char>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                position: 0,
                input: input.chars().collect(),
                scratch: Vec::new(),
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span_char(&self) -> std::ops::Range<usize> {
            self.position..self.position + 1
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position += 1; 
            self.position < self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.position]
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn error(&self, _span: std::ops::Range<usize>, _kind: ast::ErrorKind) -> Result<ast::Literal, ast::Error> {
            Err(ast::Error)
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser::new("{G}");
    let result = parser.parse_hex_brace(ast::HexLiteralKind::SomeKind);
    assert!(result.is_err());
}

#[test]
fn test_parse_hex_brace_empty_hex() {
    struct TestParser {
        position: usize,
        input: Vec<char>,
        scratch: Vec<char>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                position: 0,
                input: input.chars().collect(),
                scratch: Vec::new(),
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span_char(&self) -> std::ops::Range<usize> {
            self.position..self.position + 1
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position += 1;
            self.position < self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.position]
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn error(&self, _span: std::ops::Range<usize>, _kind: ast::ErrorKind) -> Result<ast::Literal, ast::Error> {
            Err(ast::Error)
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser::new("{ }");
    let result = parser.parse_hex_brace(ast::HexLiteralKind::SomeKind);
    assert!(result.is_err());
}

#[test]
fn test_parse_hex_brace_unexpected_eof() {
    struct TestParser {
        position: usize,
        input: Vec<char>,
        scratch: Vec<char>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                position: 0,
                input: input.chars().collect(),
                scratch: Vec::new(),
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span_char(&self) -> std::ops::Range<usize> {
            self.position..self.position + 1
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position += 1; 
            self.position < self.input.len() && self.input[self.position] != ' '
        }

        fn char(&self) -> char {
            self.input[self.position]
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn error(&self, _span: std::ops::Range<usize>, _kind: ast::ErrorKind) -> Result<ast::Literal, ast::Error> {
            Err(ast::Error)
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser::new("{00");
    let result = parser.parse_hex_brace(ast::HexLiteralKind::SomeKind);
    assert!(result.is_err());
}

#[test]
fn test_parse_hex_brace_success() {
    struct TestParser {
        position: usize,
        input: Vec<char>,
        scratch: Vec<char>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                position: 0,
                input: input.chars().collect(),
                scratch: Vec::new(),
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span_char(&self) -> std::ops::Range<usize> {
            self.position..self.position + 1
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.position += 1; 
            self.position < self.input.len() && self.input[self.position] != ' '
        }

        fn char(&self) -> char {
            self.input[self.position]
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn error(&self, _span: std::ops::Range<usize>, _kind: ast::ErrorKind) -> Result<ast::Literal, ast::Error> {
            Err(ast::Error)
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser::new("{2F}");
    let result = parser.parse_hex_brace(ast::HexLiteralKind::SomeKind);
    assert!(result.is_ok());
}

