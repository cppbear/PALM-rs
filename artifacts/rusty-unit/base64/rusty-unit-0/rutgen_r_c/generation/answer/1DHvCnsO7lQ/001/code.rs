// Answer 0

#[test]
fn test_new_general_purpose_with_valid_alphabet() {
    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };
    let config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };
    
    let engine = GeneralPurpose::new(&alphabet, config);
    
    assert_eq!(engine.encode_table, *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert_eq!(engine.config.encode_padding, true);
}

#[test]
fn test_new_general_purpose_with_empty_alphabet() {
    let alphabet = Alphabet {
        symbols: [0; 64],
    };
    let config = GeneralPurposeConfig {
        encode_padding: false,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };

    let engine = GeneralPurpose::new(&alphabet, config);

    assert_eq!(engine.encode_table, [0; 64]);
    assert_eq!(engine.config.encode_padding, false);
}

#[test]
fn test_new_general_purpose_with_custom_padding_config() {
    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.",
    };
    let config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: true,
        decode_padding_mode: DecodePaddingMode::Indifferent,
    };

    let engine = GeneralPurpose::new(&alphabet, config);
    
    assert_eq!(engine.encode_table, *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.");
    assert_eq!(engine.config.decode_padding_mode, DecodePaddingMode::Indifferent);
}

