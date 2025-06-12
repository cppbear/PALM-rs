// Answer 0

#[test]
fn test_new_with_zero_encoded_length() {
    struct Base64Decoder {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let decoder = Base64Decoder::new(0);
    assert_eq!(decoder.rem, 0);
    assert_eq!(decoder.conservative_decoded_len, 0);
}

#[test]
fn test_new_with_encoded_length_divisible_by_four() {
    struct Base64Decoder {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let decoder = Base64Decoder::new(8); // 8 is divisible by 4
    assert_eq!(decoder.rem, 0);
    assert_eq!(decoder.conservative_decoded_len, 6); // (8 / 4) * 3 = 6
}

#[test]
fn test_new_with_encoded_length_not_divisible_by_four() {
    struct Base64Decoder {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let decoder = Base64Decoder::new(10); // 10 is not divisible by 4
    assert_eq!(decoder.rem, 2); // 10 % 4 = 2
    assert_eq!(decoder.conservative_decoded_len, 9); // (10 / 4 + 1) * 3 = 9
}

#[test]
fn test_new_with_encoded_length_equal_to_three() {
    struct Base64Decoder {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let decoder = Base64Decoder::new(3); // 3 is not divisible by 4
    assert_eq!(decoder.rem, 3); // 3 % 4 = 3
    assert_eq!(decoder.conservative_decoded_len, 3); // (3 / 4 + 1) * 3 = 3
}

#[test]
fn test_new_with_large_encoded_length() {
    struct Base64Decoder {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let decoder = Base64Decoder::new(1000); // A large number
    assert_eq!(decoder.rem, 0); // 1000 % 4 = 0
    assert_eq!(decoder.conservative_decoded_len, 750); // (1000 / 4) * 3 = 750
}

