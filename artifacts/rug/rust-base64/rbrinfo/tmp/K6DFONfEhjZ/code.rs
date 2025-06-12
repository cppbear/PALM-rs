pub const fn with_encode_padding(self, padding: bool) -> Self {
        Self {
            encode_padding: padding,
            ..self
        }
    }