// Answer 0

#[test]
fn test_parse_flags_with_duplicate_flags() {
    struct TestParser {
        pos: Cell<Position>,
        capture_index: Cell<u32>,
        octal: bool,
        scratch: RefCell<String>,
        flags: Vec<char>,
        index: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            if self.index < self.flags.len() {
                self.flags[self.index]
            } else {
                ':' // Simulating that we reached the end of flags
            }
        }

        fn bump(&mut self) -> bool {
            if self.index < self.flags.len() {
                self.index += 1;
                true
            } else {
                false
            }
        }

        fn span_char(&self) -> Span {
            Span {
                start: Position { offset: self.index, line: 1, column: 1 },
                end: Position { offset: self.index + 1, line: 1, column: 2 },
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from("test_pattern"), span }
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let parser = TestParser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        octal: false,
        scratch: RefCell::new(String::new()),
        flags: vec!['i', 'i'], // Duplicate flag for 'CaseInsensitive'
        index: 0,
    };

    let result = parser.parse_flags();
    assert!(result.is_err()); // Expecting an error
    if let Err(error) = result {
        match error.kind {
            ast::ErrorKind::FlagDuplicate { original } => {
                assert_eq!(original.start.offset, 0); // Expecting the first index where 'i' was found
            },
            _ => panic!("Expected FlagDuplicate error"),
        }
    }
}

#[test]
fn test_parse_flags_with_dangling_negation() {
    struct TestParser {
        pos: Cell<Position>,
        capture_index: Cell<u32>,
        octal: bool,
        scratch: RefCell<String>,
        flags: Vec<char>,
        index: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            if self.index < self.flags.len() {
                self.flags[self.index]
            } else {
                ':' // Simulating that we reached the end of flags
            }
        }

        fn bump(&mut self) -> bool {
            if self.index < self.flags.len() {
                self.index += 1;
                true
            } else {
                false
            }
        }

        fn span_char(&self) -> Span {
            Span {
                start: Position { offset: self.index, line: 1, column: 1 },
                end: Position { offset: self.index + 1, line: 1, column: 2 },
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from("test_pattern"), span }
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let parser = TestParser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        octal: false,
        scratch: RefCell::new(String::new()),
        flags: vec!['-'], // Dangling negation
        index: 0,
    };

    let result = parser.parse_flags();
    assert!(result.is_err()); // Expecting an error
    if let Err(error) = result {
        match error.kind {
            ast::ErrorKind::FlagDanglingNegation => (),
            _ => panic!("Expected FlagDanglingNegation error"),
        }
    }
}

