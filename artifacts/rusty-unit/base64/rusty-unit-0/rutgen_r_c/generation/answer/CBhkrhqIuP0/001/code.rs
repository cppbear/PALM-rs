// Answer 0

#[test]
fn test_config_standard() {
    struct StandardEngine {
        config: GeneralPurposeConfig,
    }
    
    let config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };

    let engine = StandardEngine { config };

    assert_eq!(engine.config().encode_padding, true);
    assert_eq!(engine.config().decode_allow_trailing_bits, false);
    assert_eq!(engine.config().decode_padding_mode, DecodePaddingMode::RequireNone);
}

#[test]
fn test_config_no_pad() {
    struct NoPadEngine {
        config: GeneralPurposeConfig,
    }

    let config = GeneralPurposeConfig {
        encode_padding: false,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };

    let engine = NoPadEngine { config };

    assert_eq!(engine.config().encode_padding, false);
    assert_eq!(engine.config().decode_allow_trailing_bits, false);
    assert_eq!(engine.config().decode_padding_mode, DecodePaddingMode::RequireNone);
}

#[test]
fn test_config_url_safe() {
    struct UrlSafeEngine {
        config: GeneralPurposeConfig,
    }

    let config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: true,
        decode_padding_mode: DecodePaddingMode::Indifferent,
    };

    let engine = UrlSafeEngine { config };

    assert_eq!(engine.config().encode_padding, true);
    assert_eq!(engine.config().decode_allow_trailing_bits, true);
    assert_eq!(engine.config().decode_padding_mode, DecodePaddingMode::Indifferent);
}

#[test]
fn test_config_no_pad_indifferent() {
    struct NoPadIndifferentEngine {
        config: GeneralPurposeConfig,
    }

    let config = GeneralPurposeConfig {
        encode_padding: false,
        decode_allow_trailing_bits: true,
        decode_padding_mode: DecodePaddingMode::Indifferent,
    };

    let engine = NoPadIndifferentEngine { config };

    assert_eq!(engine.config().encode_padding, false);
    assert_eq!(engine.config().decode_allow_trailing_bits, true);
    assert_eq!(engine.config().decode_padding_mode, DecodePaddingMode::Indifferent);
}

