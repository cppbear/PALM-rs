// Answer 0

#[test]
fn test_parse_octal_valid() {
    struct MockParser {
        octal: bool,
        chars: Vec<char>,
        pos: usize,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            // Dummy implementation for demonstration
            &Parser {
                ast: ast::parse::Parser {},
                hir: hir::translate::Translator {},
            }
        }
    }

    impl MockParser {
        fn new(octal: bool, chars: Vec<char>) -> Self {
            Self { octal, chars, pos: 0 }
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.chars.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn pos(&self) -> Position {
            Position {
                offset: self.pos,
                line: 1,
                column: (self.pos + 1) as usize,
            }
        }
        
        fn pattern(&self) -> &str {
            "01234567"
        }

        fn parser(&self) -> &Parser {
            // Dummy for the sake of the test
            &Parser {
                ast: ast::parse::Parser {},
                hir: hir::translate::Translator {},
            }
        }
    }

    let mut mock_parser = MockParser::new(true, vec!['0', '7', '7']);
    let result = mock_parser.parse_octal();
    
    assert_eq!(result.kind, ast::LiteralKind::Octal);
    assert_eq!(result.c, '7');
    assert_eq!(result.span.start.offset, 0);
    assert_eq!(result.span.end.offset, 3);
}

#[test]
#[should_panic]
fn test_parse_octal_invalid_char() {
    struct MockParser {
        octal: bool,
        chars: Vec<char>,
        pos: usize,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                ast: ast::parse::Parser {},
                hir: hir::translate::Translator {},
            }
        }
    }

    impl MockParser {
        fn new(octal: bool, chars: Vec<char>) -> Self {
            Self { octal, chars, pos: 0 }
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.chars.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn pos(&self) -> Position {
            Position {
                offset: self.pos,
                line: 1,
                column: (self.pos + 1) as usize,
            }
        }
        
        fn pattern(&self) -> &str {
            "01234567"
        }

        fn parser(&self) -> &Parser {
            &Parser {
                ast: ast::parse::Parser {},
                hir: hir::translate::Translator {},
            }
        }
    }

    let mut mock_parser = MockParser::new(true, vec!['8']);
    mock_parser.parse_octal();
}

#[test]
fn test_parse_octal_three_digit() {
    struct MockParser {
        octal: bool,
        chars: Vec<char>,
        pos: usize,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                ast: ast::parse::Parser {},
                hir: hir::translate::Translator {},
            }
        }
    }

    impl MockParser {
        fn new(octal: bool, chars: Vec<char>) -> Self {
            Self { octal, chars, pos: 0 }
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.chars.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn pos(&self) -> Position {
            Position {
                offset: self.pos,
                line: 1,
                column: (self.pos + 1) as usize,
            }
        }

        fn pattern(&self) -> &str {
            "01234567"
        }

        fn parser(&self) -> &Parser {
            &Parser {
                ast: ast::parse::Parser {},
                hir: hir::translate::Translator {},
            }
        }
    }

    let mut mock_parser = MockParser::new(true, vec!['1', '5', '7']);
    let result = mock_parser.parse_octal();

    assert_eq!(result.kind, ast::LiteralKind::Octal);
    assert_eq!(result.c, '7');
    assert_eq!(result.span.start.offset, 0);
    assert_eq!(result.span.end.offset, 3);
}

