// Answer 0

#[test]
fn test_parse_hex_digits_2_digits() {
    use regex_syntax::ast::{HexLiteralKind, Literal, LiteralKind};
    use regex_syntax::parser::{Parser, Pos};

    struct TestParser {
        pos: usize,
        input: String,
        error: Option<String>,
    }

    impl TestParser {
        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> String {
            format!("Error at {:?}: {:?}", span, kind)
        }
        
        fn parser(&self) -> &Self {
            self
        }
    }
    
    let mut parser = TestParser {
        pos: 0,
        input: String::from("03"),
        error: None,
    };
    
    let kind = HexLiteralKind::new(2);
    let result = parser.parse_hex_digits(kind);
    
    assert!(result.is_ok());
    
    if let Ok(literal) = result {
        assert_eq!(literal.kind, LiteralKind::HexFixed(kind));
        assert_eq!(literal.c, ''); // Character corresponding to hex 03
    }
}

#[test]
fn test_parse_hex_digits_4_digits() {
    use regex_syntax::ast::{HexLiteralKind, Literal, LiteralKind};
    use regex_syntax::parser::{Parser, Pos};

    struct TestParser {
        pos: usize,
        input: String,
        error: Option<String>,
    }

    impl TestParser {
        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> String {
            format!("Error at {:?}: {:?}", span, kind)
        }
        
        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser {
        pos: 0,
        input: String::from("007A"),
        error: None,
    };

    let kind = HexLiteralKind::new(4);
    let result = parser.parse_hex_digits(kind);
    
    assert!(result.is_ok());

    if let Ok(literal) = result {
        assert_eq!(literal.kind, LiteralKind::HexFixed(kind));
        assert_eq!(literal.c, 'z'); // Character corresponding to hex 007A
    }
}

#[test]
fn test_parse_hex_digits_8_digits() {
    use regex_syntax::ast::{HexLiteralKind, Literal, LiteralKind};
    use regex_syntax::parser::{Parser, Pos};

    struct TestParser {
        pos: usize,
        input: String,
        error: Option<String>,
    }

    impl TestParser {
        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> String {
            format!("Error at {:?}: {:?}", span, kind)
        }
        
        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser {
        pos: 0,
        input: String::from("0000007F"),
        error: None,
    };

    let kind = HexLiteralKind::new(8);
    let result = parser.parse_hex_digits(kind);
    
    assert!(result.is_ok());

    if let Ok(literal) = result {
        assert_eq!(literal.kind, LiteralKind::HexFixed(kind));
        assert_eq!(literal.c, ''); // Character corresponding to hex 0000007F
    }
}

