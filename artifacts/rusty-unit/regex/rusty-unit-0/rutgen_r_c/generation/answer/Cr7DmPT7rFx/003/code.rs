// Answer 0

#[test]
fn test_bump_with_newline() {
    struct TestParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                pattern: pattern.to_string(),
            }
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn is_eof(&self) -> bool {
            self.pos.get().offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get().offset).unwrap_or('\0')
        }

        fn bump(&self) -> bool {
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
            self.pos.set(Position {
                offset: offset,
                line: line,
                column: column,
            });
            self.pattern[self.pos().offset..].chars().next().is_some()
        }
    }

    let parser = TestParser::new("abc\ndef");
    let result = parser.bump();
    assert!(result);
    assert_eq!(parser.pos().line, 1);
    assert_eq!(parser.pos().column, 4);
    assert_eq!(parser.pos().offset, 3);
}

#[test]
fn test_bump_with_eof() {
    struct TestParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Cell::new(Position { offset: pattern.len(), line: 1, column: 1 }),
                pattern: pattern.to_string(),
            }
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn is_eof(&self) -> bool {
            self.pos.get().offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get().offset).unwrap_or('\0')
        }

        fn bump(&self) -> bool {
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
            self.pos.set(Position {
                offset: offset,
                line: line,
                column: column,
            });
            self.pattern[self.pos().offset..].chars().next().is_some()
        }
    }

    let parser = TestParser::new("abc");
    // Move to end of input
    for _ in 0..3 {
        parser.bump();
    }
    let result = parser.bump();
    assert!(!result);
    assert_eq!(parser.pos().line, 1);
    assert_eq!(parser.pos().column, 4);
    assert_eq!(parser.pos().offset, 3);
}

