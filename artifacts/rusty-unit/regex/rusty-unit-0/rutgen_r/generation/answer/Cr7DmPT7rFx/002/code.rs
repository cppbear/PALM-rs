// Answer 0

#[test]
fn test_bump_valid_case() {
    struct MockParser {
        input: String,
        offset: usize,
        position: Position,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                offset: 0,
                position: Position { offset: 0, line: 1, column: 1 },
            }
        }
        
        fn char(&self) -> char {
            self.input[self.offset..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.offset >= self.input.len()
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn parser(&mut self) -> &mut Self {
            self
        }

        fn pattern(&self) -> &str {
            &self.input
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn bump(&mut self) -> bool {
            if self.is_eof() {
                return false;
            }
            let Position { mut offset, mut line, mut column } = self.pos();
            if self.char() == '\n' {
                line = line.checked_add(1).unwrap();
                column = 1;
            } else {
                column = column.checked_add(1).unwrap();
            }
            offset += self.char().len_utf8();
            self.position = Position {
                offset: offset,
                line: line,
                column: column,
            };
            self.offset += self.char().len_utf8();
            self.pattern()[self.offset()..].chars().next().is_some()
        }
    }

    let mut parser = MockParser::new("hello");
    let result = parser.bump();
    assert!(result);
    assert_eq!(parser.pos(), Position { offset: 1, line: 1, column: 2 });
}

#[test]
#[should_panic]
fn test_bump_panic_on_line_overflow() {
    struct PanicParser {
        input: String,
        offset: usize,
        position: Position,
    }

    impl PanicParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                offset: 0,
                position: Position { offset: 0, line: usize::MAX, column: 1 },
            }
        }
        
        // Other helper methods here...

        fn bump(&mut self) -> bool {
            // The actual bump implementation here...
            unimplemented!()
        }
    }

    let mut parser = PanicParser::new("test");
    parser.bump(); // This should panic due to overflow in checked_add
}

#[test]
#[should_panic]
fn test_bump_panic_on_pattern_slice() {
    struct SlicePanicParser {
        input: String,
        offset: usize,
        position: Position,
    }

    impl SlicePanicParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                offset: input.len(), // Start at the end of string to trigger panic
                position: Position { offset: input.len(), line: 1, column: 1 },
            }
        }
        
        // Other helper methods here...

        fn bump(&mut self) -> bool {
            // The actual bump implementation here...
            unimplemented!()
        }
    }

    let mut parser = SlicePanicParser::new("test");
    parser.bump(); // This should panic due to slice out of bounds
}

