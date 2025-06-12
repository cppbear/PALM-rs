// Answer 0

#[derive(Clone)]
struct DummyParser {
    pos: Position,
}

impl DummyParser {
    fn new(offset: usize, line: usize, column: usize) -> Self {
        DummyParser {
            pos: Position {
                offset,
                line,
                column,
            },
        }
    }
}

#[derive(Clone)]
struct Position {
    offset: usize,
    line: usize,
    column: usize,
}

struct RegexParser {
    parser: DummyParser,
}

impl RegexParser {
    fn new(offset: usize, line: usize, column: usize) -> Self {
        RegexParser {
            parser: DummyParser::new(offset, line, column),
        }
    }

    fn parser(&self) -> &DummyParser {
        &self.parser
    }

    fn pos(&self) -> Position {
        self.parser().pos.clone()
    }
}

#[test]
fn test_pos_valid() {
    let parser = RegexParser::new(10, 1, 5);
    let position = parser.pos();
    assert_eq!(position.offset, 10);
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 5);
}

#[test]
fn test_pos_boundary() {
    let parser = RegexParser::new(0, 0, 0);
    let position = parser.pos();
    assert_eq!(position.offset, 0);
    assert_eq!(position.line, 0);
    assert_eq!(position.column, 0);
}

#[test]
fn test_pos_large_values() {
    let parser = RegexParser::new(usize::MAX, usize::MAX, usize::MAX);
    let position = parser.pos();
    assert_eq!(position.offset, usize::MAX);
    assert_eq!(position.line, usize::MAX);
    assert_eq!(position.column, usize::MAX);
}

