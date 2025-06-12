// Answer 0

#[derive(Debug, Clone)]
struct Config {
    decode_allow_trailing_bits: bool,
}

impl Config {
    pub const fn new(decode_allow_trailing_bits: bool) -> Self {
        Config { decode_allow_trailing_bits }
    }

    pub const fn with_decode_allow_trailing_bits(self, allow: bool) -> Self {
        Self {
            decode_allow_trailing_bits: allow,
            ..self
        }
    }
}

#[test]
fn test_with_decode_allow_trailing_bits_update_true() {
    let config = Config::new(false);
    let updated_config = config.with_decode_allow_trailing_bits(true);
    assert_eq!(updated_config.decode_allow_trailing_bits, true);
}

#[test]
fn test_with_decode_allow_trailing_bits_update_false() {
    let config = Config::new(true);
    let updated_config = config.with_decode_allow_trailing_bits(false);
    assert_eq!(updated_config.decode_allow_trailing_bits, false);
}

#[test]
fn test_with_decode_allow_trailing_bits_no_change() {
    let config = Config::new(true);
    let updated_config = config.with_decode_allow_trailing_bits(true);
    assert_eq!(updated_config.decode_allow_trailing_bits, true);
}

