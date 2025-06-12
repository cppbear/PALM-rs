// Answer 0

#[test]
fn test_general_purpose_config_new() {
    let config = GeneralPurposeConfig::new();
    assert_eq!(config.encode_padding, true);
    assert_eq!(config.decode_allow_trailing_bits, false);
    assert_eq!(config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

#[test]
fn test_general_purpose_config_with_encode_padding() {
    let config = GeneralPurposeConfig::new().with_encode_padding(false);
    assert_eq!(config.encode_padding, false);
    assert_eq!(config.decode_allow_trailing_bits, false);
    assert_eq!(config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

#[test]
fn test_general_purpose_config_with_decode_allow_trailing_bits() {
    let config = GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true);
    assert_eq!(config.encode_padding, true);
    assert_eq!(config.decode_allow_trailing_bits, true);
    assert_eq!(config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

#[test]
fn test_general_purpose_config_with_decode_padding_mode() {
    let config = GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent);
    assert_eq!(config.encode_padding, true);
    assert_eq!(config.decode_allow_trailing_bits, false);
    assert_eq!(config.decode_padding_mode, DecodePaddingMode::Indifferent);
}

