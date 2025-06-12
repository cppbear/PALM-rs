// Answer 0

#[test]
fn test_new_alphabet_with_valid_length_and_duplicates() {
    let duplicate_byte = 65; // 'A'
    let alphabet = [
        65, 66, 67, 68, 69, 70, 71, 72,
        73, 74, 75, 76, 77, 78, 79, 80,
        81, 82, 83, 84, 85, 86, 87, 88,
        89, 90, 91, 92, 93, 94, 95, 96,
        97, 98, 99, 100, 101, 102, 103, 104,
        duplicate_byte, // Duplicate A
        106, 107, 108, 109, 110, 111, 112, 113,
        114, 115, 116, 117, 118, 119, 120, 121,
        122, 123, 124, 125, 126
    ];
    let result = Alphabet::new(std::str::from_utf8(&alphabet).unwrap());
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(duplicate_byte)));
}

#[test]
fn test_new_alphabet_with_potentially_unprintable_byte() {
    let alphabet = [
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63,
        64, 65, 66, 67, 68, 69, 70, 71,
        72, 73, 74, 75, 76, 77, 78, 79,
        80 // 'P' is unprintable as it exceeds printable characters.
    ];
    let result = Alphabet::new(std::str::from_utf8(&alphabet).unwrap());
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(80)));
}

#[test]
fn test_new_alphabet_with_invalid_length() {
    let alphabet = "A".repeat(ALPHABET_SIZE - 1); // Length is 63
    let result = Alphabet::new(&alphabet);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_alphabet_with_reserved_byte() {
    let alphabet = [
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63,
        64, 65, 66, 67, 68, 69, 70, 71,
        72, 73, 74, 75, 76, 77, 78, 79,
        PAD_BYTE // Reserved byte
    ];
    let result = Alphabet::new(std::str::from_utf8(&alphabet).unwrap());
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(PAD_BYTE)));
}

