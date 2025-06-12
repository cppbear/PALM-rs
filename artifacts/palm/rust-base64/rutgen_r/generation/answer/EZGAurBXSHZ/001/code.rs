// Answer 0

#[derive(Debug, PartialEq)]
enum DecodePaddingMode {
    RequireCanonical,
}

struct Config {
    encode_padding: bool,
    decode_allow_trailing_bits: bool,
    decode_padding_mode: DecodePaddingMode,
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
fn test_new_config() {
    let expected = Config {
        encode_padding: true,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireCanonical,
    };
    
    let config = Config::new();
    
    assert_eq!(config, expected);
}

