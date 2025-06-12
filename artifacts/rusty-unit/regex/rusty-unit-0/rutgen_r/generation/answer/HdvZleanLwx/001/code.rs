// Answer 0

#[test]
fn test_parse_octal_valid_input() {
    struct Parser {
        octal: bool,
        pos: usize,
        pattern: String,
    }
    
    impl Parser {
        fn new(octal: bool, pattern: &str) -> Self {
            Self {
                octal,
                pos: 0,
                pattern: pattern.to_string(),
            }
        }
        
        fn bump(&mut self) -> bool {
            if self.pos < self.pattern.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }
        
        fn char(&self) -> char {
            self.pattern[self.pos..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Pos {
            Pos { offset: self.pos }
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    struct Pos {
        offset: usize,
    }

    let mut parser = Parser::new(true, "077");
    
    let start = parser.pos();
    assert!(parser.char() >= '0' && parser.char() <= '7');
    
    let literal = parse_octal(&mut parser);
    
    let end = parser.pos();
    assert_eq!(literal.span.offset, end.offset);
    assert_eq!(literal.kind, ast::LiteralKind::Octal);
    assert_eq!(literal.c, ''); // 511 in Unicode
}

#[test]
#[should_panic]
fn test_parse_octal_invalid_input_non_octal_start() {
    struct Parser {
        octal: bool,
        pos: usize,
        pattern: String,
    }

    impl Parser {
        fn new(octal: bool, pattern: &str) -> Self {
            Self {
                octal,
                pos: 0,
                pattern: pattern.to_string(),
            }
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.pattern.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.pos..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Pos {
            Pos { offset: self.pos }
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    struct Pos {
        offset: usize,
    }

    let mut parser = Parser::new(true, "8"); // invalid octal input
    
    let start = parser.pos();
    
    parse_octal(&mut parser); // this should panic
}

#[test]
#[should_panic]
fn test_parse_octal_exceeds_3_digits() {
    struct Parser {
        octal: bool,
        pos: usize,
        pattern: String,
    }

    impl Parser {
        fn new(octal: bool, pattern: &str) -> Self {
            Self {
                octal,
                pos: 0,
                pattern: pattern.to_string(),
            }
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.pattern.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.pos..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Pos {
            Pos { offset: self.pos }
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    struct Pos {
        offset: usize,
    }

    let mut parser = Parser::new(true, "0788"); // too many octal digits
    
    let start = parser.pos();
    
    parse_octal(&mut parser); // this should panic
}

#[test]
fn test_parse_octal_valid_input_with_bump() {
    struct Parser {
        octal: bool,
        pos: usize,
        pattern: String,
    }

    impl Parser {
        fn new(octal: bool, pattern: &str) -> Self {
            Self {
                octal,
                pos: 0,
                pattern: pattern.to_string(),
            }
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.pattern.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.pos..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Pos {
            Pos { offset: self.pos }
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    struct Pos {
        offset: usize,
    }

    let mut parser = Parser::new(true, "0777"); 
    
    let start = parser.pos();
    assert!(parser.char() >= '0' && parser.char() <= '7');
    
    // Perform a bump to simulate parsing
    parser.bump(); 
    
    let literal = parse_octal(&mut parser);
    
    let end = parser.pos();
    assert_eq!(literal.span.offset, end.offset);
    assert_eq!(literal.kind, ast::LiteralKind::Octal);
    assert_eq!(literal.c, ''); // 511 in Unicode
}

