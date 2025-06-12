pub const fn with_decode_allow_trailing_bits(self, allow: bool) -> Self {
        Self {
            decode_allow_trailing_bits: allow,
            ..self
        }
    }