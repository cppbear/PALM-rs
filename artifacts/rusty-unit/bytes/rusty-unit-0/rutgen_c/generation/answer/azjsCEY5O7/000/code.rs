// Answer 0

#[test]
fn test_is_unique_static_slice() {
    let bytes = Bytes::from_static(&[1, 2, 3]);
    assert!(!bytes.is_unique());
}

#[test]
fn test_is_unique_owned_bytes() {
    let bytes = Bytes::from_owner(vec![1, 2, 3]);
    assert!(bytes.is_unique());
    
    let _clone = bytes.clone();
    assert!(!bytes.is_unique());
}

#[test]
fn test_is_unique_after_clone() {
    let mut bytes = Bytes::from(vec![1, 2, 3]);
    assert!(bytes.is_unique());
    
    let _ = bytes.clone();
    assert!(!bytes.is_unique());
}

