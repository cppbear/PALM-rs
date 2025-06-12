// Answer 0

#[test]
fn test_parse_escape_valid_literal() {
    struct MockParser {
        pos: Position,
        pattern: String,
        octal: bool,
    }

    impl MockParser {
        fn char(&self) -> char {
            '\\'
        }
        
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            panic!("Error occurred")
        }
        
        fn parser(&self) -> &MockParser {
            self
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump() // No space handling in the mock version
        }

        fn is_eof(&self) -> bool {
            false // Simulate not at end of file
        }

        fn ignore_whitespace(&self) -> bool {
            false // No whitespace ignoring in the test
        }
    }

    let mut parser = MockParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: String::from(r"\n"),
        octal: false,
    };
    
    match parser.parse_escape() {
        Ok(result) => match result {
            Primitive::Literal(lit) => {
                assert_eq!(lit.c, '\n'); // Expect newline character for \n
                assert_eq!(lit.kind, LiteralKind::Special(SpecialLiteralKind::LineFeed));
            },
            _ => panic!("Expected a Literal")
        },
        Err(_) => panic!("Expected successful parsing"),
    }
}

#[test]
fn test_parse_escape_valid_perl_class() {
    struct MockParser {
        pos: Position,
        pattern: String,
        octal: bool,
    }

    impl MockParser {
        fn char(&self) -> char {
            '\\'
        }
        
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            panic!("Error occurred")
        }
        
        fn parser(&self) -> &MockParser {
            self
        }
        
        fn is_eof(&self) -> bool {
            false // Simulate not at end of file
        }
        
        fn bump_and_bump_space(&mut self) -> bool {
            self.bump() // No space handling in the mock version
        }

        fn ignore_whitespace(&self) -> bool {
            false // No whitespace ignoring in the test
        }
    }

    let mut parser = MockParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: String::from(r"\d"),
        octal: false,
    };
    
    match parser.parse_escape() {
        Ok(result) => match result {
            Primitive::Perl(cls) => {
                assert_eq!(cls.kind, ast::ClassPerlKind::Digit);
                assert_eq!(cls.negated, false);
            },
            _ => panic!("Expected a Perl class")
        },
        Err(_) => panic!("Expected successful parsing"),
    }
}

#[test]
fn test_parse_escape_unrecognized_escape() {
    struct MockParser {
        pos: Position,
        pattern: String,
        octal: bool,
    }

    impl MockParser {
        fn char(&self) -> char {
            '\\'
        }
        
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            panic!("Unrecognized escape sequence")
        }
        
        fn parser(&self) -> &MockParser {
            self
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump() // No space handling in the mock version
        }

        fn is_eof(&self) -> bool {
            false // Simulate not at end of file
        }
        
        fn ignore_whitespace(&self) -> bool {
            false // No whitespace ignoring in the test
        }
    }

    let mut parser = MockParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: String::from(r"\x"),
        octal: false,
    };
    
    let result = parser.parse_escape();
    assert!(result.is_err()); // Expect an error due to unrecognized escape
} 

#[test]
fn test_parse_escape_invalid_octal() {
    struct MockParser {
        pos: Position,
        pattern: String,
        octal: bool,
    }

    impl MockParser {
        fn char(&self) -> char {
            '\\'
        }
        
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            panic!("Unsupported backreference due to octal")
        }
        
        fn parser(&self) -> &MockParser {
            self
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump() // No space handling in the mock version
        }

        fn is_eof(&self) -> bool {
            false // Simulate not at end of file
        }
        
        fn ignore_whitespace(&self) -> bool {
            false // No whitespace ignoring in the test
        }
    }

    let mut parser = MockParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: String::from(r"\8"),
        octal: false,
    };

    let result = parser.parse_escape();
    assert!(result.is_err()); // Expect an error due to unsupported backreference
} 

#[test]
fn test_parse_escape_valid_special() {
    struct MockParser {
        pos: Position,
        pattern: String,
        octal: bool,
    }

    impl MockParser {
        fn char(&self) -> char {
            '\\'
        }
        
        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn error(&self, _span: Span, _kind: ErrorKind) -> Error {
            panic!("Error occurred")
        }
        
        fn parser(&self) -> &MockParser {
            self
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump() // No space handling in the mock version
        }

        fn is_eof(&self) -> bool {
            false // Simulate not at end of file
        }
        
        fn ignore_whitespace(&self) -> bool {
            false // No whitespace ignoring in the test
        }
    }

    let mut parser = MockParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: String::from(r"\a"),
        octal: false,
    };

    match parser.parse_escape() {
        Ok(result) => match result {
            Primitive::Literal(lit) => {
                assert_eq!(lit.c, '\x07'); // Expect bell character for \a
                assert_eq!(lit.kind, LiteralKind::Special(SpecialLiteralKind::Bell));
            },
            _ => panic!("Expected a Literal"),
        },
        Err(_) => panic!("Expected successful parsing"),
    }
}

