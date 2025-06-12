// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self { input, pos: 0 }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {}
        }

        fn pattern(&self) -> &str {
            self.input
        }

        fn add_capture_name(&self, _name: &ast::CaptureName) -> Result<(), ast::Error> {
            Ok(())
        }
    }

    let mut parser = MockParser::new("<valid_name>");
    let result = parser.parse_capture_name(1);
    assert!(result.is_ok());
    let capname = result.unwrap();

    assert_eq!(capname.name, "valid_name");
    assert_eq!(capname.index, 1);
}

#[test]
#[should_panic]
fn test_parse_capture_name_empty() {
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self { input, pos: 0 }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {}
        }

        fn pattern(&self) -> &str {
            self.input
        }

        fn add_capture_name(&self, _name: &ast::CaptureName) -> Result<(), ast::Error> {
            Ok(())
        }
    }

    let mut parser = MockParser::new("<>");
    let _ = parser.parse_capture_name(1);
}

#[test]
fn test_parse_capture_name_invalid_char() {
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self { input, pos: 0 }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {}
        }

        fn pattern(&self) -> &str {
            self.input
        }

        fn add_capture_name(&self, _name: &ast::CaptureName) -> Result<(), ast::Error> {
            Ok(())
        }
    }

    let mut parser = MockParser::new("<valid_name!");
    let result = parser.parse_capture_name(1);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_parse_capture_name_eof_before_close() {
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self { input, pos: 0 }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() { return '\0'; }
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {}
        }

        fn pattern(&self) -> &str {
            self.input
        }

        fn add_capture_name(&self, _name: &ast::CaptureName) -> Result<(), ast::Error> {
            Ok(())
        }
    }

    let mut parser = MockParser::new("<valid_name");
    let _ = parser.parse_capture_name(1);
}

