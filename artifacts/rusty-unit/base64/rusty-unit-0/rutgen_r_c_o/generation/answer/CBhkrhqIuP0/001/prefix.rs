// Answer 0

#[test]
fn test_config_standard() {
    let engine = GeneralPurpose {
        encode_table: [0; 64],
        decode_table: [0; 256],
        config: PAD,
    };
    let _config = engine.config();
}

#[test]
fn test_config_standard_no_pad() {
    let engine = GeneralPurpose {
        encode_table: [0; 64],
        decode_table: [0; 256],
        config: NO_PAD,
    };
    let _config = engine.config();
}

#[test]
fn test_config_url_safe() {
    let engine = GeneralPurpose {
        encode_table: [0; 64],
        decode_table: [0; 256],
        config: URL_SAFE,
    };
    let _config = engine.config();
}

#[test]
fn test_config_url_safe_no_pad() {
    let engine = GeneralPurpose {
        encode_table: [0; 64],
        decode_table: [0; 256],
        config: URL_SAFE_NO_PAD,
    };
    let _config = engine.config();
}

