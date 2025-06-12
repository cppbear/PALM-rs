// Answer 0

#[test]
fn test_zeroed_creates_bytes_mut_with_correct_length_and_capacity() {
    let len = 42;
    let zeros = BytesMut::zeroed(len);

    assert!(zeros.capacity() >= len);
    assert_eq!(zeros.len(), len);
}

#[test]
fn test_zeroed_creates_bytes_mut_with_zeroed_contents() {
    let len = 10;
    let zeros = BytesMut::zeroed(len);

    let slice: &[u8] = unsafe { zeros.as_slice() };
    for &byte in slice.iter() {
        assert_eq!(byte, 0);
    }
}

#[test]
fn test_zeroed_with_zero_length_creates_empty_bytes_mut() {
    let zeros = BytesMut::zeroed(0);

    assert!(zeros.capacity() >= 0);
    assert_eq!(zeros.len(), 0);
}

