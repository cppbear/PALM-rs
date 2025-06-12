// Answer 0

#[test]
fn test_empty_string() {
    let deserializer = BorrowedStrDeserializer::new("");
}

#[test]
fn test_single_character_string() {
    let deserializer = BorrowedStrDeserializer::new("a");
}

#[test]
fn test_longer_string() {
    let deserializer = BorrowedStrDeserializer::new("longer string with characters");
}

#[test]
fn test_unicode_string() {
    let deserializer = BorrowedStrDeserializer::new("特殊字符");
}

#[test]
fn test_utf8_encoded_string() {
    let deserializer = BorrowedStrDeserializer::new(std::str::from_utf8(&[0xE2, 0x9C, 0x94]).unwrap());
}

