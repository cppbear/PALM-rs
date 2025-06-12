fn peek_error(&self, reason: ErrorCode) -> Error {
        let position = self.read.peek_position();
        Error::syntax(reason, position.line, position.column)
    }