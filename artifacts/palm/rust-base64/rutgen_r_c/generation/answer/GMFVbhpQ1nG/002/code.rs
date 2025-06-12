// Answer 0

#[test]
fn test_new_alphabet_invalid_reserved_byte() {
    const PAD_BYTE: u8 = b'='; // Assuming PAD_BYTE is defined as the '=' character
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    // Change the last character to the PAD_BYTE ('=') to trigger the reserved byte error
    let invalid_input = input.chars().take(63).collect::<String>() + "=";

    let result = Alphabet::new(&invalid_input);
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(PAD_BYTE)));
}

#[test]
fn test_new_alphabet_too_short() {
    let too_short_input = "ABCD"; // Shorter than ALPHABET_SIZE
    let result = Alphabet::new(too_short_input);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_alphabet_too_long() {
    let too_long_input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/somethingextra"; // Longer than ALPHABET_SIZE
    let result = Alphabet::new(too_long_input);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_alphabet_with_unprintable_byte() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // Valid input
    let unprintable_input = input.chars().take(63).collect::<String>() + "\x01"; // Append an unprintable character

    let result = Alphabet::new(&unprintable_input);
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(1)));
}

#[test]
fn test_new_alphabet_with_duplicate_bytes() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // Valid input
    let duplicate_byte_input = input.chars().take(62).collect::<String>() + "AB"; // Append duplicate 'A' and 'B'

    let result = Alphabet::new(&duplicate_byte_input);
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

