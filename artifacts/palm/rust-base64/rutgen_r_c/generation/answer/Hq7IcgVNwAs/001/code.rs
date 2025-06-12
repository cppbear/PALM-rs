// Answer 0

#[test]
fn test_with_decode_allow_trailing_bits_true() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_allow_trailing_bits(true);
    assert_eq!(updated_config.decode_allow_trailing_bits, true);
    assert_eq!(updated_config.encode_padding, config.encode_padding);
    assert_eq!(updated_config.decode_padding_mode, config.decode_padding_mode);
}

#[test]
fn test_with_decode_allow_trailing_bits_false() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_allow_trailing_bits(false);
    assert_eq!(updated_config.decode_allow_trailing_bits, false);
    assert_eq!(updated_config.encode_padding, config.encode_padding);
    assert_eq!(updated_config.decode_padding_mode, config.decode_padding_mode);
}

#[test]
fn test_with_decode_allow_trailing_bits_on_modified_config() {
    let config = GeneralPurposeConfig::new()
        .with_encode_padding(false)
        .with_decode_padding_mode(DecodePaddingMode::Indifferent);
    let updated_config = config.with_decode_allow_trailing_bits(true);
    assert_eq!(updated_config.decode_allow_trailing_bits, true);
    assert_eq!(updated_config.encode_padding, false);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::Indifferent);
}

