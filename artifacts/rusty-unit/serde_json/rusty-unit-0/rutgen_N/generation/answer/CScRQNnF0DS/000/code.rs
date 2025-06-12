// Answer 0

#[derive(Debug)]
struct Iter {
    line_number: usize,
    column_number: usize,
}

impl Iter {
    fn line(&self) -> usize {
        self.line_number
    }

    fn col(&self) -> usize {
        self.column_number
    }
}

struct Reader {
    iter: Iter,
}

impl Reader {
    fn new(line_number: usize, column_number: usize) -> Self {
        Reader {
            iter: Iter {
                line_number,
                column_number,
            },
        }
    }

    fn position(&self) -> Position {
        Position {
            line: self.iter.line(),
            column: self.iter.col(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    line: usize,
    column: usize,
}

#[test]
fn test_position() {
    let reader = Reader::new(3, 5);
    let pos = reader.position();
    assert_eq!(pos, Position { line: 3, column: 5 });
}

#[test]
fn test_position_with_zero_values() {
    let reader = Reader::new(0, 0);
    let pos = reader.position();
    assert_eq!(pos, Position { line: 0, column: 0 });
}

#[test]
fn test_position_with_large_values() {
    let reader = Reader::new(usize::MAX, usize::MAX);
    let pos = reader.position();
    assert_eq!(pos, Position { line: usize::MAX, column: usize::MAX });
}

