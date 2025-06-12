// Answer 0

#[test]
fn test_from_slice_valid_bytes() {
    let bytes = b"{\"key\":\"value\"}"; // valid JSON
    let deserializer = Deserializer::from_slice(bytes);
    assert!(deserializer.read.index == 0);
}

#[test]
fn test_from_slice_empty_bytes() {
    let bytes: &[u8] = &[];
    let deserializer = Deserializer::from_slice(bytes);
    assert!(deserializer.read.index == 0);
}

#[test]
fn test_from_slice_non_empty_bytes() {
    let bytes = b"[1, 2, 3]"; // valid JSON array
    let deserializer = Deserializer::from_slice(bytes);
    assert!(deserializer.read.index == 0);
}

