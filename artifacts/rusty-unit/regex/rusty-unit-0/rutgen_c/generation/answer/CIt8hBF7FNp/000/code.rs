// Answer 0

#[test]
fn test_parse_set_class_open_valid_case() {
    struct MockParser<'s> {
        input: &'s str,
        pos: Position,
    }

    impl<'s> MockParser<'s> {
        fn new(input: &'s str) -> Self {
            MockParser {
                input,
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.input[self.pos.offset..].chars().next().unwrap_or('\0')
        }
        
        fn pos(&self) -> Position {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos.offset += 1; 
            self.pos.line = 1;
            self.pos.column += 1;
            true
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Result<Ast> {
            Err(Error { kind, pattern: self.input.to_string(), span })
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let mut mock_parser = MockParser::new("[abc]");
    
    let (class, union) = mock_parser.parse_set_class_open().unwrap();
    
    assert_eq!(class.negated, false);
    assert!(union.items.is_empty());
}

#[test]
fn test_parse_set_class_open_negated_case() {
    struct MockParser<'s> {
        input: &'s str,
        pos: Position,
    }

    impl<'s> MockParser<'s> {
        fn new(input: &'s str) -> Self {
            MockParser {
                input,
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.input[self.pos.offset..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos.offset += 1; 
            self.pos.line = 1;
            self.pos.column += 1;
            true
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Result<Ast> {
            Err(Error { kind, pattern: self.input.to_string(), span })
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let mut mock_parser = MockParser::new("[^abc]");
    
    let (class, union) = mock_parser.parse_set_class_open().unwrap();
    
    assert_eq!(class.negated, true);
    assert!(union.items.is_empty());
}

#[test]
#[should_panic]
fn test_parse_set_class_open_unclosed_case() {
    struct MockParser<'s> {
        input: &'s str,
        pos: Position,
    }

    impl<'s> MockParser<'s> {
        fn new(input: &'s str) -> Self {
            MockParser {
                input,
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.input[self.pos.offset..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos.offset += 1; 
            self.pos.line = 1;
            self.pos.column += 1;
            false 
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Result<Ast> {
            Err(Error { kind, pattern: self.input.to_string(), span })
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let mut mock_parser = MockParser::new("[abc");
    mock_parser.parse_set_class_open().unwrap();
}

