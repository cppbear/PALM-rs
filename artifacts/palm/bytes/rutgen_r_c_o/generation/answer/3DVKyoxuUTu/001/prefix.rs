// Answer 0

#[test]
fn test_advance_zero() {
    let mut bytes = Bytes::from_static(b"test");
    bytes.advance(0);
}

#[test]
fn test_advance_full_length() {
    let mut bytes = Bytes::from_static(b"example");
    bytes.advance(bytes.len());
}

#[test]
#[should_panic]
fn test_advance_exceeding_length() {
    let mut bytes = Bytes::from_static(b"data");
    bytes.advance(bytes.len() + 1);
}

#[test]
fn test_advance_at_length() {
    let mut bytes = Bytes::from_static(b"edge case");
    bytes.advance(bytes.len());
}

