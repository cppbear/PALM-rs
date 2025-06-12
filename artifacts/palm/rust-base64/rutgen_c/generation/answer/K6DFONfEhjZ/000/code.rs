// Answer 0

#[test]
fn test_with_encode_padding_true() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_encode_padding(true);
    assert!(updated_config.encode_padding);
}

#[test]
fn test_with_encode_padding_false() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_encode_padding(false);
    assert!(!updated_config.encode_padding);
}

#[test]
fn test_with_encode_padding_boundary() {
    let config1 = GeneralPurposeConfig::new().with_encode_padding(true);
    let updated_config1 = config1.with_encode_padding(false);
    assert!(!updated_config1.encode_padding);

    let config2 = GeneralPurposeConfig::new().with_encode_padding(false);
    let updated_config2 = config2.with_encode_padding(true);
    assert!(updated_config2.encode_padding);
}

