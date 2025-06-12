// Answer 0

#[test]
fn test_advance_with_valid_count() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(10, 0u8);
    bytes_mut.advance(5);
    assert_eq!(bytes_mut.len(), 5);
    assert_eq!(bytes_mut.capacity(), 10);
}

#[test]
#[should_panic(expected = "cannot advance past `remaining`")]
fn test_advance_with_exceeding_count() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(10, 0u8);
    bytes_mut.advance(11); // Should panic
}

#[test]
fn test_advance_with_zero_count() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(10, 0u8);
    bytes_mut.advance(0);
    assert_eq!(bytes_mut.len(), 10);
    assert_eq!(bytes_mut.capacity(), 10);
}

#[test]
fn test_advance_to_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(10, 0u8);
    bytes_mut.advance(10);
    assert_eq!(bytes_mut.len(), 0);
    assert_eq!(bytes_mut.capacity(), 10);
}

