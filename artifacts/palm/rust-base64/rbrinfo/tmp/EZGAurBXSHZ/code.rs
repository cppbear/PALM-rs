pub const fn new() -> Self {
        Self {
            // RFC states that padding must be applied by default
            encode_padding: true,
            decode_allow_trailing_bits: false,
            decode_padding_mode: DecodePaddingMode::RequireCanonical,
        }
    }