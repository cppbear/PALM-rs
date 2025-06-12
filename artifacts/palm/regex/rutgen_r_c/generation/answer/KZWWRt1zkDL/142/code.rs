// Answer 0

#[test]
fn test_parse_escape_perl_class_digit() {
    struct MockParser {
        pattern: String,
        position: Position,
        octal: bool,
    }
    
    impl MockParser {
        fn new(pattern: &str, octal: bool) -> Self {
            Self {
                pattern: pattern.to_string(),
                position: Position { offset: 0, line: 1, column: 1 },
                octal,
            }
        }

        // Mimic bumped position
        fn bump(&mut self) -> bool {
            self.position.offset += 1;
            self.position.column += 1;
            if self.position.offset < self.pattern.len() {
                return true;
            }
            false
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.position.offset).unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.position.clone()
        }

        fn parser(&self) -> &Self {
            self
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error {
                kind: kind,
                pattern: self.pattern.clone(),
                span: span,
            }
        }

        fn parse_perl_class(&self) -> ClassPerl {
            let c = self.char();
            self.bump(); // simulate bump
            let (negated, kind) = match c {
                'd' => (false, ClassPerlKind::Digit),
                'D' => (true, ClassPerlKind::Digit),
                's' => (false, ClassPerlKind::Space),
                'S' => (true, ClassPerlKind::Space),
                'w' => (false, ClassPerlKind::Word),
                'W' => (true, ClassPerlKind::Word),
                _ => panic!("expected valid Perl class but got '{}'", c),
            };
            ClassPerl {
                span: Span::new(self.pos(), self.pos()),
                kind,
                negated,
            }
        }
    }

    let mut parser = MockParser::new(r"\d", false);
    parser.bump(); // Skip over the backslash
    let result = parser.parse_perl_class();
    
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert_eq!(result.negated, false);
}

#[test]
fn test_parse_escape_perl_class_word() {
    struct MockParser {
        pattern: String,
        position: Position,
        octal: bool,
    }
    
    impl MockParser {
        fn new(pattern: &str, octal: bool) -> Self {
            Self {
                pattern: pattern.to_string(),
                position: Position { offset: 0, line: 1, column: 1 },
                octal,
            }
        }

        fn bump(&mut self) -> bool {
            self.position.offset += 1;
            self.position.column += 1;
            self.position.offset < self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.position.offset).unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.position.clone()
        }

        fn parser(&self) -> &Self {
            self
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error {
                kind: kind,
                pattern: self.pattern.clone(),
                span: span,
            }
        }

        fn parse_perl_class(&self) -> ClassPerl {
            let c = self.char();
            self.bump(); // simulate bump
            let (negated, kind) = match c {
                'w' => (false, ClassPerlKind::Word),
                'W' => (true, ClassPerlKind::Word),
                _ => panic!("expected valid Perl class but got '{}'", c),
            };
            ClassPerl {
                span: Span::new(self.pos(), self.pos()),
                kind,
                negated,
            }
        }
    }

    let mut parser = MockParser::new(r"\w", false);
    parser.bump(); // Skip over the backslash
    let result = parser.parse_perl_class();
    
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert_eq!(result.negated, false);
}

#[test]
fn test_parse_escape_perl_class_space() {
    struct MockParser {
        pattern: String,
        position: Position,
        octal: bool,
    }
    
    impl MockParser {
        fn new(pattern: &str, octal: bool) -> Self {
            Self {
                pattern: pattern.to_string(),
                position: Position { offset: 0, line: 1, column: 1 },
                octal,
            }
        }

        fn bump(&mut self) -> bool {
            self.position.offset += 1;
            self.position.column += 1;
            self.position.offset < self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.position.offset).unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.position.clone()
        }

        fn parser(&self) -> &Self {
            self
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error {
                kind: kind,
                pattern: self.pattern.clone(),
                span: span,
            }
        }

        fn parse_perl_class(&self) -> ClassPerl {
            let c = self.char();
            self.bump(); // simulate bump
            let (negated, kind) = match c {
                's' => (false, ClassPerlKind::Space),
                'S' => (true, ClassPerlKind::Space),
                _ => panic!("expected valid Perl class but got '{}'", c),
            };
            ClassPerl {
                span: Span::new(self.pos(), self.pos()),
                kind,
                negated,
            }
        }
    }

    let mut parser = MockParser::new(r"\s", false);
    parser.bump(); // Skip over the backslash
    let result = parser.parse_perl_class();
    
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert_eq!(result.negated, false);
}

