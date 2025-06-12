// Answer 0

#[test]
fn test_parse_escape_octal() {
    struct MockParser {
        octal: bool,
    }

    struct MockInput {
        input: Vec<char>,
        pos: usize,
    }

    impl MockInput {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos + 1 < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&self) -> &MockParser {
            &MockParser { octal: true }
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> String {
            "error".to_string()
        }

        fn parse_octal(&mut self) -> ast::Literal {
            ast::Literal {
                span: Span::new(0, 0), // Use dummy value
                kind: ast::LiteralKind::Punctuation, // Use dummy kind
                c: '0', // Just return a simple octal character
            }
        }
        
        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut input = MockInput::new(r"\7");
    let result = input.parse_escape();
    
    assert!(result.is_ok());
    match result {
        Ok(Primitive::Literal(lit)) => {
            assert_eq!(lit.c, '0'); // Just checking a dummy value for c
        },
        _ => panic!("Expected a Primitive::Literal"),
    }
}

#[test]
#[should_panic]
fn test_parse_escape_octal_empty() {
    struct MockParser {
        octal: bool,
    }

    struct MockInput {
        input: Vec<char>,
        pos: usize,
    }

    impl MockInput {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos + 1 < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&self) -> &MockParser {
            &MockParser { octal: true }
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> String {
            "error".to_string()
        }

        fn parse_octal(&mut self) -> ast::Literal {
            // This will panic since input is empty
            ast::Literal {
                span: Span::new(0, 0),
                kind: ast::LiteralKind::Punctuation,
                c: '0',
            }
        }
        
        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let input = MockInput::new(r"\");
    input.parse_escape();
}

#[test]
fn test_parse_escape_octal_not_supported() {
    struct MockParser {
        octal: bool,
    }

    struct MockInput {
        input: Vec<char>,
        pos: usize,
    }

    impl MockInput {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos + 1 < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&self) -> &MockParser {
            &MockParser { octal: false }
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> String {
            "error".to_string()
        }

        fn parse_octal(&mut self) -> ast::Literal {
            ast::Literal {
                span: Span::new(0, 0),
                kind: ast::LiteralKind::Punctuation,
                c: '0',
            }
        }
        
        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut input = MockInput::new(r"\8");
    let result = input.parse_escape();
    
    assert!(result.is_err());
}

