// Answer 0

#[test]
fn test_with_decode_padding_mode_indifferent() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_padding_mode(DecodePaddingMode::Indifferent);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::Indifferent);
}

#[test]
fn test_with_decode_padding_mode_require_canonical() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_padding_mode(DecodePaddingMode::RequireCanonical);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

#[test]
fn test_with_decode_padding_mode_require_none() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_padding_mode(DecodePaddingMode::RequireNone);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireNone);
}

#[test]
fn test_with_decode_padding_mode_multiple_updates() {
    let config = GeneralPurposeConfig::new();
    let updated_config_indifferent = config.with_decode_padding_mode(DecodePaddingMode::Indifferent);
    let updated_config_require_none = updated_config_indifferent.with_decode_padding_mode(DecodePaddingMode::RequireNone);
    assert_eq!(updated_config_require_none.decode_padding_mode, DecodePaddingMode::RequireNone);
}

