pub const fn with_decode_padding_mode(self, mode: DecodePaddingMode) -> Self {
        Self {
            decode_padding_mode: mode,
            ..self
        }
    }