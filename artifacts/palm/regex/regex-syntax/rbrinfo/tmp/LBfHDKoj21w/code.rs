pub fn with_end(self, pos: Position) -> Span {
        Span { end: pos, ..self }
    }