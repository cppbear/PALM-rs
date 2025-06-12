// Answer 0

#[test]
fn test_general_purpose_new() {
    struct TestAlphabet {
        symbols: [u8; 64],
    }

    impl Alphabet for TestAlphabet {
        // Dummy method implementations if required by trait
    }

    let test_alphabet = TestAlphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };
    let test_config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };

    let engine = GeneralPurpose::new(&test_alphabet, test_config);
    assert_eq!(engine.encode_table, encode_table(&test_alphabet));
    assert_eq!(engine.decode_table, decode_table(&test_alphabet));
    assert_eq!(engine.config.encode_padding, test_config.encode_padding);
    assert_eq!(engine.config.decode_allow_trailing_bits, test_config.decode_allow_trailing_bits);
    assert_eq!(engine.config.decode_padding_mode, test_config.decode_padding_mode);
}

#[test]
fn test_general_purpose_new_with_different_config() {
    struct TestAlphabet {
        symbols: [u8; 64],
    }

    impl Alphabet for TestAlphabet {
        // Dummy method implementations if required by trait
    }

    let test_alphabet = TestAlphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };
    let test_config = GeneralPurposeConfig {
        encode_padding: false,
        decode_allow_trailing_bits: true,
        decode_padding_mode: DecodePaddingMode::Indifferent,
    };

    let engine = GeneralPurpose::new(&test_alphabet, test_config);
    assert_eq!(engine.encode_table, encode_table(&test_alphabet));
    assert_eq!(engine.decode_table, decode_table(&test_alphabet));
    assert_eq!(engine.config.encode_padding, test_config.encode_padding);
    assert_eq!(engine.config.decode_allow_trailing_bits, test_config.decode_allow_trailing_bits);
    assert_eq!(engine.config.decode_padding_mode, test_config.decode_padding_mode);
}

#[test]
fn test_general_purpose_new_invalid_alphabet() {
    struct InvalidAlphabet {
        symbols: [u8; 64],
    }

    impl Alphabet for InvalidAlphabet {
        // Dummy method implementations if required by trait
    }

    let invalid_alphabet = InvalidAlphabet {
        symbols: *b"INVALID_ALPHABET_IS_TOO_SHORT_",
    };
    let test_config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };

    let engine = GeneralPurpose::new(&invalid_alphabet, test_config);
    // Assuming the encode_table and decode_table will still hold but with invalid data
    assert_ne!(engine.encode_table, [0; 64]);
    assert_ne!(engine.decode_table, [255; 256]);
}

