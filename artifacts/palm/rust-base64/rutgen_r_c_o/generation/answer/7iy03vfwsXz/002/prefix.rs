// Answer 0

#[test]
fn test_fmt_unprintable_byte_low() {
    let error = ParseAlphabetError::UnprintableByte(0u8);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", error);
}

#[test]
fn test_fmt_unprintable_byte_high() {
    let error = ParseAlphabetError::UnprintableByte(128u8);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", error);
}

#[test]
fn test_fmt_unprintable_byte_mid() {
    let error = ParseAlphabetError::UnprintableByte(31u8);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", error);
}

#[test]
fn test_fmt_unprintable_byte_extreme_high() {
    let error = ParseAlphabetError::UnprintableByte(255u8);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", error);
}

