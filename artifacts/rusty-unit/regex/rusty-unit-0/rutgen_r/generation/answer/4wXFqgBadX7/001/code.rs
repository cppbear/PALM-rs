// Answer 0

#[derive(Default)]
struct Position {
    column: usize,
}

struct Parser {
    pos: Position,
}

impl Parser {
    fn new() -> Self {
        Parser {
            pos: Position::default(),
        }
    }

    fn reset_column(&mut self) {
        self.pos.column = 1;
    }

    fn advance_column(&mut self) {
        self.pos.column += 1;
    }

    fn new_line(&mut self) {
        self.reset_column();
    }

    fn column(&self) -> usize {
        self.pos.column
    }
}

#[test]
fn test_column_initial() {
    let parser = Parser::new();
    assert_eq!(parser.column(), 1);
}

#[test]
fn test_column_after_advance() {
    let mut parser = Parser::new();
    parser.advance_column();
    parser.advance_column();
    assert_eq!(parser.column(), 3);
}

#[test]
fn test_column_after_new_line() {
    let mut parser = Parser::new();
    parser.advance_column();
    parser.new_line();
    assert_eq!(parser.column(), 1);
}

#[test]
fn test_column_resets_on_new_line() {
    let mut parser = Parser::new();
    parser.advance_column();
    parser.new_line();
    parser.advance_column();
    assert_eq!(parser.column(), 2); // Should be 2 after advancing again
}

