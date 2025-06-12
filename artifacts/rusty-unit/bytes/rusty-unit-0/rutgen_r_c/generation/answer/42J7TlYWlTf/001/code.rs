// Answer 0

#[test]
fn test_len_empty() {
    let b = Bytes::new();
    assert_eq!(b.len(), 0);
}

#[test]
fn test_len_non_empty() {
    let b = Bytes::from_static(&[1, 2, 3, 4, 5]);
    assert_eq!(b.len(), 5);
}

#[test]
fn test_len_single_byte() {
    let b = Bytes::from_static(&[42]);
    assert_eq!(b.len(), 1);
}

#[test]
fn test_len_large_array() {
    let large_array = [0u8; 1024];
    let b = Bytes::from_static(&large_array);
    assert_eq!(b.len(), 1024);
}

#[test]
fn test_len_zero_length_slice() {
    let b = Bytes::from_static(&[]);
    assert_eq!(b.len(), 0);
}

