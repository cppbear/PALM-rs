fn __span(&self) -> Span {
        join_spans(self.into_token_stream())
    }