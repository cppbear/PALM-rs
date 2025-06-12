// Answer 0

#[test]
fn test_from_str_unchecked_valid_input() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let alphabet_instance = Alphabet::from_str_unchecked(alphabet);
    let expected_symbols = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
        b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
        b'8', b'9', b'+', b'/',
    ];
    assert_eq!(alphabet_instance.symbols, expected_symbols);
}

#[should_panic]
#[test]
fn test_from_str_unchecked_exceeding_length() {
    let alphabet = "A"; // Length is not 64
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[should_panic]
#[test]
fn test_from_str_unchecked_empty_string() {
    let alphabet = ""; // Length is not 64
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[should_panic]
#[test]
fn test_from_str_unchecked_unprintable_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x00"; // Contains unprintable byte
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[should_panic]
#[test]
fn test_from_str_unchecked_reserved_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="; // Contains reserved byte (PAD_BYTE)
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[test]
fn test_from_str_unchecked_boundary_condition() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let alphabet_instance = Alphabet::from_str_unchecked(alphabet);
    let expected_symbols = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
        b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
        b'8', b'9', b'-', b'_',
    ];
    assert_eq!(alphabet_instance.symbols, expected_symbols);
}

