// Answer 0

#[test]
fn test_bump_eof() {
    struct Parser {
        input: String,
        offset: usize,
    }

    impl Parser {
        fn pos(&self) -> Position {
            let line = self.input[..self.offset].matches('\n').count() as usize + 1;
            let column = self.offset - self.input[..self.offset].rfind('\n').map_or(0, |n| n + 1) + 1;
            Position { offset: self.offset, line, column }
        }

        fn char(&self) -> char {
            self.input[self.offset..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.offset >= self.input.len()
        }

        fn bump(&mut self) -> bool {
            if self.is_eof() {
                return false;
            }
            let Position { mut offset, mut line, mut column } = self.pos();
            if self.char() == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            offset += self.char().len_utf8();
            self.offset = offset;
            self.char().is_some() // We check if the next character is present
        }
    }

    struct Position {
        offset: usize,
        line: usize,
        column: usize,
    }

    let mut parser = Parser { input: String::from("abc\ndef"), offset: 0 };

    assert!(parser.bump());
    assert_eq!(parser.offset, 1);
    assert_eq!(parser.pos().line, 1);
    assert_eq!(parser.pos().column, 2);

    assert!(parser.bump());
    assert_eq!(parser.offset, 2);
    assert_eq!(parser.pos().line, 1);
    assert_eq!(parser.pos().column, 3);

    assert!(parser.bump());
    assert_eq!(parser.offset, 3);
    assert_eq!(parser.pos().line, 1);
    assert_eq!(parser.pos().column, 4);

    assert!(parser.bump());
    assert_eq!(parser.offset, 4);
    assert_eq!(parser.pos().line, 1);
    assert_eq!(parser.pos().column, 5);

    assert!(parser.bump()); // moves to newline
    assert_eq!(parser.offset, 5);
    assert_eq!(parser.pos().line, 2);
    assert_eq!(parser.pos().column, 1);

    assert!(parser.bump());
    assert_eq!(parser.offset, 6);
    assert_eq!(parser.pos().line, 2);
    assert_eq!(parser.pos().column, 2);

    assert!(parser.bump());
    assert_eq!(parser.offset, 7);
    assert_eq!(parser.pos().line, 2);
    assert_eq!(parser.pos().column, 3);

    assert!(!parser.bump()); // should return false as we hit EOF
}

#[test]
fn test_bump_empty_input() {
    struct Parser {
        input: String,
        offset: usize,
    }

    impl Parser {
        fn pos(&self) -> Position {
            let line = self.input[..self.offset].matches('\n').count() as usize + 1;
            let column = self.offset - self.input[..self.offset].rfind('\n').map_or(0, |n| n + 1) + 1;
            Position { offset: self.offset, line, column }
        }

        fn char(&self) -> char {
            self.input[self.offset..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.offset >= self.input.len()
        }

        fn bump(&mut self) -> bool {
            if self.is_eof() {
                return false;
            }
            let Position { mut offset, mut line, mut column } = self.pos();
            if self.char() == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            offset += self.char().len_utf8();
            self.offset = offset;
            self.char().is_some()
        }
    }

    struct Position {
        offset: usize,
        line: usize,
        column: usize,
    }

    let mut parser = Parser { input: String::new(), offset: 0 };

    assert!(!parser.bump()); // should return false as input is empty
}

