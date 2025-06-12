// Answer 0

#[test]
fn test_internal_encode_exact_blocks() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder {
        encode_table: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };

    let input: &[u8] = b"Man";
    let mut output = [0u8; 8];
    let encoded_length = encoder.internal_encode(input, &mut output);
    assert_eq!(encoded_length, 8);
    assert_eq!(&output[..encoded_length], b"TWFu");
}

#[test]
fn test_internal_encode_with_padding_1() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder {
        encode_table: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };

    let input: &[u8] = b"Ma";
    let mut output = [0u8; 8];
    let encoded_length = encoder.internal_encode(input, &mut output);
    assert_eq!(encoded_length, 4);
    assert_eq!(&output[..encoded_length], b"TWE");
}

#[test]
fn test_internal_encode_with_padding_2() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder {
        encode_table: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };

    let input: &[u8] = b"M";
    let mut output = [0u8; 8];
    let encoded_length = encoder.internal_encode(input, &mut output);
    assert_eq!(encoded_length, 4);
    assert_eq!(&output[..encoded_length], b"MQ==");
}

#[test]
fn test_internal_encode_empty_input() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder {
        encode_table: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };

    let input: &[u8] = b"";
    let mut output = [0u8; 8];
    let encoded_length = encoder.internal_encode(input, &mut output);
    assert_eq!(encoded_length, 0);
}

#[test]
fn test_internal_encode_full_input() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder {
        encode_table: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };

    let input: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef";
    let mut output = [0u8; 48];
    let encoded_length = encoder.internal_encode(input, &mut output);
    assert_eq!(encoded_length, 64);
    assert_eq!(&output[..encoded_length], b"QUJDREQ1Nzg5MTAyMzQ1Njc4OUFCTENqJTI2MDAw");
}

