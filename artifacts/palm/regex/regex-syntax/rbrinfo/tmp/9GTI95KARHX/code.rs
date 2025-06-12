pub fn new(offset: usize, line: usize, column: usize) -> Position {
        Position { offset: offset, line: line, column: column }
    }