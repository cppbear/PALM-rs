// Answer 0

#[derive(Default)]
struct MockParser {
    input: String,
    position: usize,
}

impl MockParser {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
        }
    }

    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    fn char(&self) -> char {
        if self.is_eof() {
            '\0' // EOF character
        } else {
            self.input.chars().nth(self.position).unwrap()
        }
    }

    fn bump(&mut self) -> bool {
        if self.is_eof() {
            false
        } else {
            self.position += 1;
            true
        }
    }

    fn pos(&self) -> usize {
        self.position
    }

    fn pattern(&self) -> &str {
        &self.input
    }
    
    fn span(&self) -> usize {
        self.position
    }
    
    fn span_char(&self) -> usize {
        self.position
    }

    fn error(&self, _: usize, error_kind: ast::ErrorKind) -> ast::Result<ast::CaptureName> {
        Err(error_kind) // Simplified for testing purposes
    }

    fn add_capture_name(&self, _capname: &ast::CaptureName) -> ast::Result<()> {
        Ok(()) // Simplified for testing purposes
    }
}

#[test]
fn test_parse_capture_name_invalid_character() {
    let mut parser = MockParser::new("<*>");
    
    let result = parser.parse_capture_name(1);
    
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error, ast::ErrorKind::GroupNameInvalid);
    }
}

#[test]
fn test_parse_capture_name_eof_before_close() {
    let mut parser = MockParser::new("<invalid");
    
    let result = parser.parse_capture_name(1);
    
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error, ast::ErrorKind::GroupNameUnexpectedEof);
    }
}

