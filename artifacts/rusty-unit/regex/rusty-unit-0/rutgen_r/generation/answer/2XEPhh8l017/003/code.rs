// Answer 0

#[derive(Debug)]
struct MockParser {
    current_pos: usize,
    input: Vec<char>,
}

impl MockParser {
    fn new(input: &str) -> Self {
        MockParser {
            current_pos: 0,
            input: input.chars().collect(),
        }
    }

    fn char(&self) -> char {
        self.input[self.current_pos]
    }

    fn pos(&self) -> usize {
        self.current_pos
    }

    fn bump_and_bump_space(&mut self) -> bool {
        self.current_pos += 1;
        self.current_pos < self.input.len() // Simulate a successful bump
    }

    fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
        ast::Error { span, kind }
    }

    fn span_char(&self) -> Span {
        Span { start: self.current_pos, end: self.current_pos + 1 }
    }

    fn parser(&self) -> &Self {
        self
    }
}

#[derive(Debug)]
struct MockHexLiteralKind {
    digits: usize,
}

impl MockHexLiteralKind {
    fn digits(&self) -> usize {
        self.digits
    }
}

#[test]
fn test_parse_hex_digits_invalid() {
    let mut parser = MockParser::new("a");
    let kind = MockHexLiteralKind { digits: 2 };
    
    let result = parser.parse_hex_digits(kind);
    
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::EscapeHexInvalidDigit);
    }
}

#[test]
fn test_parse_hex_digits_with_multiple_inputs() {
    let mut parser = MockParser::new("z"); // 'z' is not a hex digit
    let kind = MockHexLiteralKind { digits: 4 };
    
    let result = parser.parse_hex_digits(kind);
    
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::EscapeHexInvalidDigit);
    }
}

#[test]
fn test_parse_hex_digits_bump_and_space() {
    let mut parser = MockParser::new("1g"); // 'g' is not a hex digit
    let kind = MockHexLiteralKind { digits: 2 };

    let result = parser.parse_hex_digits(kind);
    
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::EscapeHexInvalidDigit);
    }
}

