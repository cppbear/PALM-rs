// Answer 0

#[test]
fn test_new_with_single_zero() {
    let deserializer = BorrowedBytesDeserializer::new(&[0u8]);
}

#[test]
fn test_new_with_array_of_one_zero() {
    let deserializer = BorrowedBytesDeserializer::new(&[0u8; 1]);
}

#[test]
fn test_new_with_single_max_value() {
    let deserializer = BorrowedBytesDeserializer::new(&[255u8]);
}

#[test]
fn test_new_with_empty_slice() {
    let deserializer = BorrowedBytesDeserializer::new(&[]);
}

#[test]
fn test_new_with_multiple_values() {
    let deserializer = BorrowedBytesDeserializer::new(&[1, 2, 3, 4]);
}

