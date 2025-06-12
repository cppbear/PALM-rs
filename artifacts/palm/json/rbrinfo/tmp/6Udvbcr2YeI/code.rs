fn peek_position(&self) -> Position {
        // The LineColIterator updates its position during peek() so it has the
        // right one here.
        self.position()
    }