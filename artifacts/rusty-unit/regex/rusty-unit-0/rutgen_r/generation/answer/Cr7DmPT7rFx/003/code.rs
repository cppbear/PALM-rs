// Answer 0

#[test]
fn test_bump_newline() {
    struct TestParser {
        offset: usize,
        input: String,
        pos: Position,
    }

    impl TestParser {
        fn is_eof(&self) -> bool {
            self.offset >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.offset..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn parser(&mut self) -> &mut TestParser {
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
            self.pos = Position { offset, line, column };
            self.pattern()[self.offset()..].chars().next().is_some()
        }
    }

    #[derive(Clone)]
    struct Position {
        offset: usize,
        line: usize,
        column: usize,
    }

    let mut parser = TestParser {
        offset: 0,
        input: "Hello\nWorld".to_string(),
        pos: Position { offset: 0, line: 1, column: 1 },
    };

    assert_eq!(parser.bump(), true);
    assert_eq!(parser.pos().line, 1);
    assert_eq!(parser.pos().column, 1);
    
    parser.offset += 6; // Move past "Hello\n"
    assert_eq!(parser.bump(), true);
    assert_eq!(parser.pos().line, 2);
    assert_eq!(parser.pos().column, 1);
    
    parser.offset += 5; // Move past "World"
    assert_eq!(parser.bump(), false);
}

