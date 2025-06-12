fn error(&self, reason: ErrorCode) -> Error {
        let position = self.read.position();
        Error::syntax(reason, position.line, position.column)
    }