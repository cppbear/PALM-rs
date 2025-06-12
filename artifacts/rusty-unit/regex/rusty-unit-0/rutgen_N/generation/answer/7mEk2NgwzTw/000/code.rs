// Answer 0

#[derive(Default)]
struct Parser {
    pos: Position,
}

#[derive(Default)]
struct Position {
    line: usize,
}

struct TestParser {
    inner: Parser,
}

impl TestParser {
    fn new(line: usize) -> Self {
        Self { 
            inner: Parser {
                pos: Position { line },
            },
        }
    }
    
    fn line(&self) -> usize {
        self.inner.pos.line
    }
}

#[test]
fn test_line_start() {
    let parser = TestParser::new(1);
    assert_eq!(parser.line(), 1);
}

#[test]
fn test_line_mid() {
    let parser = TestParser::new(10);
    assert_eq!(parser.line(), 10);
}

#[test]
fn test_line_end() {
    let parser = TestParser::new(100);
    assert_eq!(parser.line(), 100);
}

