// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    struct MockParser {
        pattern: String,
        position: usize,
        end_of_file: bool,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            MockParser {
                pattern: pattern.to_string(),
                position: 0,
                end_of_file: false,
            }
        }

        fn is_eof(&self) -> bool {
            self.end_of_file
        }

        fn char(&self) -> char {
            if self.position >= self.pattern.len() {
                return '>';
            }
            self.pattern.chars().nth(self.position).unwrap_or('>')
        }

        fn bump(&mut self) -> bool {
            if self.position < self.pattern.len() {
                self.position += 1;
                return true;
            }
            false
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span(&self) -> usize {
            self.position
        }

        fn span_char(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn add_capture_name(&self, _: &ast::CaptureName) -> Result<(), &'static str> {
            Ok(())
        }

        fn error(&self, _: usize, _: ast::ErrorKind) -> &'static str {
            "error"
        }
    }

    let mut parser = MockParser::new("validName>");
    let capture_index = 0;

    let result = parser.parse_capture_name(capture_index);
    
    assert!(result.is_ok());
    let capname = result.unwrap();
    assert_eq!(capname.name, "validName");
    assert_eq!(capname.index, capture_index);
}

#[test]
fn test_parse_capture_name_empty() {
    struct MockParser {
        pattern: String,
        position: usize,
        end_of_file: bool,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            MockParser {
                pattern: pattern.to_string(),
                position: 0,
                end_of_file: false,
            }
        }

        fn is_eof(&self) -> bool {
            self.end_of_file
        }

        fn char(&self) -> char {
            if self.position >= self.pattern.len() {
                return '>';
            }
            self.pattern.chars().nth(self.position).unwrap_or('>')
        }

        fn bump(&mut self) -> bool {
            if self.position < self.pattern.len() {
                self.position += 1;
                return true;
            }
            false
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span(&self) -> usize {
            self.position
        }

        fn span_char(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn add_capture_name(&self, _: &ast::CaptureName) -> Result<(), &'static str> {
            Ok(())
        }

        fn error(&self, _: usize, _: ast::ErrorKind) -> &'static str {
            "error"
        }
    }

    let mut parser = MockParser::new(">"); // empty name
    let capture_index = 0;

    let result = parser.parse_capture_name(capture_index);
    
    assert!(result.is_err());
}

#[test]
fn test_parse_capture_name_invalid() {
    struct MockParser {
        pattern: String,
        position: usize,
        end_of_file: bool,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            MockParser {
                pattern: pattern.to_string(),
                position: 0,
                end_of_file: false,
            }
        }

        fn is_eof(&self) -> bool {
            self.end_of_file
        }

        fn char(&self) -> char {
            if self.position >= self.pattern.len() {
                return '>';
            }
            self.pattern.chars().nth(self.position).unwrap_or('>')
        }

        fn bump(&mut self) -> bool {
            if self.position < self.pattern.len() {
                self.position += 1;
                return true;
            }
            false
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span(&self) -> usize {
            self.position
        }

        fn span_char(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn add_capture_name(&self, _: &ast::CaptureName) -> Result<(), &'static str> {
            Ok(())
        }

        fn error(&self, _: usize, _: ast::ErrorKind) -> &'static str {
            "error"
        }
    }

    let mut parser = MockParser::new("invalid@name>"); // invalid character
    let capture_index = 0;

    let result = parser.parse_capture_name(capture_index);
    
    assert!(result.is_err());
}

