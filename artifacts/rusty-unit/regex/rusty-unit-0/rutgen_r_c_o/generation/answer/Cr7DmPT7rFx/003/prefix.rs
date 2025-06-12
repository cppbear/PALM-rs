// Answer 0

#[derive(Clone, Debug)]
struct TestParser {
    pos: Cell<Position>,
    pattern: String,
}

impl TestParser {
    fn new() -> Self {
        TestParser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            pattern: String::new(),
        }
    }

    fn bump(&self) -> bool {
        if self.is_eof() {
            return false;
        }
        let Position { mut offset, mut line, mut column } = self.pos.get();
        if self.char() == '\n' {
            line = line.checked_add(1).unwrap();
            column = 1;
        } else {
            column = column.checked_add(1).unwrap();
        }
        offset += self.char().len_utf8();
        self.pos.set(Position { offset, line, column });
        self.pattern[self.offset()..].chars().next().is_some()
    }

    fn is_eof(&self) -> bool {
        self.offset() == self.pattern.len()
    }

    fn char(&self) -> char {
        self.char_at(self.offset())
    }

    fn char_at(&self, i: usize) -> char {
        self.pattern.chars().nth(i).unwrap_or('\0')
    }

    fn offset(&self) -> usize {
        self.pos.get().offset
    }
}

#[test]
fn test_bump_with_newline() {
    let parser = TestParser::new();
    parser.pattern = String::from("Hello\nWorld");
    parser.pos.set(Position { offset: 5, line: 1, column: 6 }); // Position at character after "Hello"
    assert_eq!(parser.bump(), true);
}

#[test]
fn test_bump_with_eof() {
    let parser = TestParser::new();
    parser.pattern = String::from("Hello\nWorld");
    parser.pos.set(Position { offset: 11, line: 2, column: 6 }); // Position at EOF
    assert_eq!(parser.bump(), false);
}

#[test]
fn test_bump_line_increment() {
    let parser = TestParser::new();
    parser.pattern = String::from("Line1\nLine2");
    parser.pos.set(Position { offset: 6, line: 1, column: 7 }); // Position at character after '\n'
    assert_eq!(parser.bump(), true);
    assert_eq!(parser.pos.get().line, 2);
    assert_eq!(parser.pos.get().column, 1);
}

#[test]
fn test_bump_column_increment() {
    let parser = TestParser::new();
    parser.pattern = String::from("Hello World");
    parser.pos.set(Position { offset: 5, line: 1, column: 6 }); // Position at character after "Hello"
    assert_eq!(parser.bump(), true);
    assert_eq!(parser.pos.get().line, 1);
    assert_eq!(parser.pos.get().column, 7);
}

