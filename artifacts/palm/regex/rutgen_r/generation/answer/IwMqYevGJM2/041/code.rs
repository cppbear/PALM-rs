// Answer 0

#[test]
fn test_parse_unicode_class_single() {
    struct TestParser<'a> {
        input: &'a str,
        pos: usize,
    }

    impl<'a> TestParser<'a> {
        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char().is_whitespace() {
                self.pos += 1;
            }
            if self.char() != '\0' {
                self.pos += 1;
                return true;
            }
            false
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> String {
            "Error".to_string()
        }
    }
    
    let mut parser = TestParser { input: "pX", pos: 0 };
    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
}

#[test]
fn test_parse_unicode_class_named() {
    struct TestParser<'a> {
        input: &'a str,
        pos: usize,
    }

    impl<'a> TestParser<'a> {
        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char().is_whitespace() {
                self.pos += 1;
            }
            if self.char() != '\0' {
                self.pos += 1;
                return true;
            }
            false
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> String {
            "Error".to_string()
        }
    }
    
    let mut parser = TestParser { input: "p{Greek}", pos: 0 };
    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
}

#[test]
fn test_parse_unicode_class_negated() {
    struct TestParser<'a> {
        input: &'a str,
        pos: usize,
    }

    impl<'a> TestParser<'a> {
        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char().is_whitespace() {
                self.pos += 1;
            }
            if self.char() != '\0' {
                self.pos += 1;
                return true;
            }
            false
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> String {
            "Error".to_string()
        }
    }
    
    let mut parser = TestParser { input: "P{Greek}", pos: 0 };
    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_unicode_class_empty() {
    struct TestParser<'a> {
        input: &'a str,
        pos: usize,
    }

    impl<'a> TestParser<'a> {
        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char().is_whitespace() {
                self.pos += 1;
            }
            if self.char() != '\0' {
                self.pos += 1;
                return true;
            }
            false
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> String {
            "Error".to_string()
        }
    }
    
    let mut parser = TestParser { input: "P{", pos: 0 };
    parser.parse_unicode_class().unwrap();
}

#[test]
#[should_panic]
fn test_parse_unicode_class_missing_closing_brace() {
    struct TestParser<'a> {
        input: &'a str,
        pos: usize,
    }

    impl<'a> TestParser<'a> {
        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char().is_whitespace() {
                self.pos += 1;
            }
            if self.char() != '\0' {
                self.pos += 1;
                return true;
            }
            false
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> String {
            "Error".to_string()
        }
    }
    
    let mut parser = TestParser { input: "P{Greek", pos: 0 };
    parser.parse_unicode_class().unwrap();
}

