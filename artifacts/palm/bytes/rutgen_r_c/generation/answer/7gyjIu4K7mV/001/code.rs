// Answer 0

#[test]
fn test_copy_to_bytes_valid_length() {
    let mut bytes_instance = Bytes::from_static(&[1, 2, 3, 4, 5]);
    let result = bytes_instance.copy_to_bytes(3);
    assert_eq!(result.len(), 3);
    assert_eq!(result.as_slice(), &[1, 2, 3]);
    assert_eq!(bytes_instance.len(), 2);
}

#[test]
#[should_panic(expected = "split_to out of bounds: 6 <= 5")]
fn test_copy_to_bytes_exceeding_length() {
    let mut bytes_instance = Bytes::from_static(&[1, 2, 3, 4, 5]);
    bytes_instance.copy_to_bytes(6);
}

#[test]
fn test_copy_to_bytes_zero_length() {
    let mut bytes_instance = Bytes::from_static(&[1, 2, 3, 4, 5]);
    let result = bytes_instance.copy_to_bytes(0);
    assert_eq!(result.len(), 0);
    assert!(result.is_empty());
    assert_eq!(bytes_instance.len(), 5);
} 

#[test]
fn test_copy_to_bytes_full_length() {
    let mut bytes_instance = Bytes::from_static(&[1, 2, 3, 4, 5]);
    let result = bytes_instance.copy_to_bytes(5);
    assert_eq!(result.len(), 5);
    assert_eq!(result.as_slice(), &[1, 2, 3, 4, 5]);
    assert!(bytes_instance.is_empty());
} 

#[test]
fn test_copy_to_bytes_empty() {
    let mut bytes_instance = Bytes::new();
    let result = bytes_instance.copy_to_bytes(0);
    assert_eq!(result.len(), 0);
    assert!(result.is_empty());
    assert!(bytes_instance.is_empty());
}

