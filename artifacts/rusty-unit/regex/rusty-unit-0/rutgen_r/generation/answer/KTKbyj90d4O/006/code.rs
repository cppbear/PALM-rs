// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.pos..].chars().next().unwrap()
            }
        }

        fn bump(&mut self) -> bool {
            if self.is_eof() {
                false
            } else {
                self.pos += self.char().len_utf8();
                true
            }
        }

        fn span(&self) -> usize {
            // Just for demonstration, returning a stubbed value
            self.pos
        }

        fn add_capture_name(&self, _: &ast::CaptureName) -> Result<(), ast::Error> {
            Ok(())
        }

        fn error(&self, _: usize, _: ast::ErrorKind) -> ast::Error {
            ast::Error {}
        }

        fn pattern(&self) -> &'static str {
            self.input
        }
    }

    let mut parser = MockParser { input: "<myCaptureName>", pos: 1 }; // Start after '<'
    let result = parser.parse_capture_name(1);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_capture_name_eof_on_start() {
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.pos..].chars().next().unwrap()
            }
        }

        fn bump(&mut self) -> bool {
            if self.is_eof() {
                false
            } else {
                self.pos += self.char().len_utf8();
                true
            }
        }

        fn span(&self) -> usize {
            // Just for demonstration, returning a stubbed value
            self.pos
        }

        fn add_capture_name(&self, _: &ast::CaptureName) -> Result<(), ast::Error> {
            Ok(())
        }

        fn error(&self, _: usize, _: ast::ErrorKind) -> ast::Error {
            ast::Error {}
        }

        fn pattern(&self) -> &'static str {
            self.input
        }
    }

    let mut parser = MockParser { input: "<invalidCaptureName>", pos: 1 }; // Start after '<'
    let _result = parser.parse_capture_name(1);
}

#[test]
fn test_parse_capture_name_empty() {
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.pos..].chars().next().unwrap()
            }
        }

        fn bump(&mut self) -> bool {
            if self.is_eof() {
                false
            } else {
                self.pos += self.char().len_utf8();
                true
            }
        }

        fn span(&self) -> usize {
            // Just for demonstration, returning a stubbed value
            self.pos
        }

        fn add_capture_name(&self, _: &ast::CaptureName) -> Result<(), ast::Error> {
            Ok(())
        }

        fn error(&self, _: usize, _: ast::ErrorKind) -> ast::Error {
            ast::Error {}
        }

        fn pattern(&self) -> &'static str {
            self.input
        }
    }

    let mut parser = MockParser { input: "<>", pos: 1 }; // Start after '<'
    let result = parser.parse_capture_name(1);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_parse_capture_name_invalid_char() {
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.pos..].chars().next().unwrap()
            }
        }

        fn bump(&mut self) -> bool {
            if self.is_eof() {
                false
            } else {
                self.pos += self.char().len_utf8();
                true
            }
        }

        fn span(&self) -> usize {
            // Just for demonstration, returning a stubbed value
            self.pos
        }

        fn add_capture_name(&self, _: &ast::CaptureName) -> Result<(), ast::Error> {
            Ok(())
        }

        fn error(&self, _: usize, _: ast::ErrorKind) -> ast::Error {
            ast::Error {}
        }

        fn pattern(&self) -> &'static str {
            self.input
        }
    }

    let mut parser = MockParser { input: "<my@CaptureName>", pos: 1 }; // Start after '<'
    let _result = parser.parse_capture_name(1);
}

