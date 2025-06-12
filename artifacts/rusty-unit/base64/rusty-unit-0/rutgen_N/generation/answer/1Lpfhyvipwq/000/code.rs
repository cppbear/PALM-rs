// Answer 0

#[derive(Debug, Clone)]
struct Config {
    decode_padding_mode: DecodePaddingMode,
}

#[derive(Debug, Clone, Copy)]
enum DecodePaddingMode {
    RequireCanonicalPadding,
    RequireNoPadding,
    Indifferent,
}

impl Config {
    pub const fn new(decode_padding_mode: DecodePaddingMode) -> Self {
        Self { decode_padding_mode }
    }

    pub const fn with_decode_padding_mode(self, mode: DecodePaddingMode) -> Self {
        Self {
            decode_padding_mode: mode,
            ..self
        }
    }
}

#[test]
fn test_with_decode_padding_mode() {
    let initial_config = Config::new(DecodePaddingMode::RequireCanonicalPadding);
    let updated_config = initial_config.with_decode_padding_mode(DecodePaddingMode::Indifferent);
    
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::Indifferent);
}

#[test]
fn test_with_no_padding_mode() {
    let initial_config = Config::new(DecodePaddingMode::RequireCanonicalPadding);
    let updated_config = initial_config.with_decode_padding_mode(DecodePaddingMode::RequireNoPadding);
    
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireNoPadding);
}

#[test]
fn test_with_indifferent_padding_mode() {
    let initial_config = Config::new(DecodePaddingMode::RequireCanonicalPadding);
    let updated_config = initial_config.with_decode_padding_mode(DecodePaddingMode::Indifferent);
    
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::Indifferent);
}

