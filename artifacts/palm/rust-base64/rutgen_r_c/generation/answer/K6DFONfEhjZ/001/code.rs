// Answer 0

#[test]
fn test_with_encode_padding_true() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_encode_padding(true);
    assert_eq!(updated_config.encode_padding, true);
    assert_eq!(updated_config.decode_allow_trailing_bits, false);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

#[test]
fn test_with_encode_padding_false() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_encode_padding(false);
    assert_eq!(updated_config.encode_padding, false);
    assert_eq!(updated_config.decode_allow_trailing_bits, false);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

#[test]
fn test_with_encode_padding_on_modified_config() {
    let config = GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(true)
        .with_decode_padding_mode(DecodePaddingMode::Indifferent);
    let updated_config = config.with_encode_padding(true);
    assert_eq!(updated_config.encode_padding, true);
    assert_eq!(updated_config.decode_allow_trailing_bits, true);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::Indifferent);
}

#[test]
fn test_with_encode_padding_on_config_with_no_pad() {
    let config = NO_PAD;
    let updated_config = config.with_encode_padding(true);
    assert_eq!(updated_config.encode_padding, true);
    assert_eq!(updated_config.decode_allow_trailing_bits, false);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireNone);
}

