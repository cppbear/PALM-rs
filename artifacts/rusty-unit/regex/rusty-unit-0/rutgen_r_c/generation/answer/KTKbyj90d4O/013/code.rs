// Answer 0

#[test]
fn test_parse_capture_name_empty() {
    struct MockParserI {
        pos: Position,
        pattern: String,
    }
    
    impl Borrow<Parser> for MockParserI {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(self.pos),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let parser = MockParserI {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: "<>".to_string(),
    };

    let result = parser.parse_capture_name(0);
    
    assert_eq!(result, Err(parser.error(
        Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        ast::ErrorKind::GroupNameEmpty,
    )));
}

#[test]
fn test_parse_capture_name_invalid_character() {
    struct MockParserI {
        pos: Position,
        pattern: String,
        current_char: char,
        eof: bool,
    }

    impl MockParserI {
        fn is_eof(&self) -> bool {
            self.eof
        }
        fn char(&self) -> char {
            self.current_char
        }
        fn bump(&mut self) -> bool {
            false
        }
        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }
    }
    
    impl Borrow<Parser> for MockParserI {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(self.pos),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let parser = MockParserI {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: "<a>".to_string(),
        current_char: '!', // Invalid character
        eof: false,
    };

    let result = parser.parse_capture_name(0);
    assert_eq!(result, Err(parser.error(
        Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        ast::ErrorKind::GroupNameInvalid,
    )));
}

#[test]
fn test_parse_capture_name_eof_before_closing() {
    struct MockParserI {
        pos: Position,
        pattern: String,
        eof: bool,
        char_pos: usize,
    }

    impl MockParserI {
        fn is_eof(&self) -> bool {
            self.eof
        }
        fn char(&self) -> char {
            if self.char_pos < self.pattern.len() {
                self.pattern[self.char_pos..self.char_pos + 1].chars().next().unwrap()
            } else {
                '\0' // Simulate EOF character
            }
        }
        fn bump(&mut self) -> bool {
            let old_pos = self.char_pos;
            self.char_pos += 1;
            old_pos < self.pattern.len()
        }
        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }
    }
    
    impl Borrow<Parser> for MockParserI {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(self.pos),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let parser = MockParserI {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: "<capture".to_string(), // No closing '>'
        eof: false,
        char_pos: 7,
    };

    let result = parser.parse_capture_name(0);
    
    assert_eq!(result, Err(parser.error(
        Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        ast::ErrorKind::GroupNameUnexpectedEof,
    )));
}

