fn error(&self, span: Span, kind: ErrorKind) -> Error {
        Error { kind: kind, pattern: self.pattern.to_string(), span: span }
    }