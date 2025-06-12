// Answer 0

#[derive(Debug, PartialEq)]
pub struct Config {
    encode_padding: bool,
    decode_allow_trailing_bits: bool,
    decode_padding_mode: DecodePaddingMode,
}

#[derive(Debug, PartialEq)]
pub enum DecodePaddingMode {
    RequireCanonical,
}

impl Config {
    pub const fn new() -> Self {
        Self {
            encode_padding: true,
            decode_allow_trailing_bits: false,
            decode_padding_mode: DecodePaddingMode::RequireCanonical,
        }
    }
}

#[test]
fn test_config_new() {
    let config = Config::new();

    assert_eq!(config.encode_padding, true);
    assert_eq!(config.decode_allow_trailing_bits, false);
    assert_eq!(config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

