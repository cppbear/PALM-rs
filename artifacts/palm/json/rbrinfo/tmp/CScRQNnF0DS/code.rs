fn position(&self) -> Position {
        Position {
            line: self.iter.line(),
            column: self.iter.col(),
        }
    }