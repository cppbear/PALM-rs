// Answer 0

#[test]
fn test_from_shared_too_long() {
    let long_bytes = Bytes::from(vec![0u8; 65536]);
    let result = Uri::from_shared(long_bytes);
}

#[test]
fn test_from_shared_exceeding_max_length() {
    let exceeding_bytes = Bytes::from(vec![0u8; 65537]);
    let result = Uri::from_shared(exceeding_bytes);
}

