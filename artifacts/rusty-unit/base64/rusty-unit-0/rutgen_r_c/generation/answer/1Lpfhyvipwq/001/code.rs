// Answer 0

#[test]
fn test_with_decode_padding_mode_indifferent() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_padding_mode(DecodePaddingMode::Indifferent);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::Indifferent);
    assert_eq!(updated_config.encode_padding, config.encode_padding);
    assert_eq!(updated_config.decode_allow_trailing_bits, config.decode_allow_trailing_bits);
}

#[test]
fn test_with_decode_padding_mode_require_canonical() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_padding_mode(DecodePaddingMode::RequireCanonical);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
    assert_eq!(updated_config.encode_padding, config.encode_padding);
    assert_eq!(updated_config.decode_allow_trailing_bits, config.decode_allow_trailing_bits);
}

#[test]
fn test_with_decode_padding_mode_require_none() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_padding_mode(DecodePaddingMode::RequireNone);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireNone);
    assert_eq!(updated_config.encode_padding, config.encode_padding);
    assert_eq!(updated_config.decode_allow_trailing_bits, config.decode_allow_trailing_bits);
}

