pub const fn new(alphabet: &Alphabet, config: GeneralPurposeConfig) -> Self {
        Self {
            encode_table: encode_table(alphabet),
            decode_table: decode_table(alphabet),
            config,
        }
    }