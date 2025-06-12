pub fn with_start(self, pos: Position) -> Span {
        Span { start: pos, ..self }
    }