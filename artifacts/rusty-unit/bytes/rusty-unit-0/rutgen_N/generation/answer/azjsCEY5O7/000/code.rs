// Answer 0

#[test]
fn test_is_unique_with_owned_bytes() {
    use bytes::Bytes;

    let a = Bytes::from(vec![1, 2, 3]);
    assert!(a.is_unique());
    let b = a.clone();
    assert!(!a.is_unique());
}

#[test]
fn test_is_unique_with_static_bytes() {
    use bytes::Bytes;

    let a = Bytes::from_static(&[1, 2, 3]);
    assert!(!a.is_unique());
}

#[test]
fn test_is_unique_with_owner_bytes() {
    use bytes::Bytes;

    let a = Bytes::from_owner(vec![4, 5, 6]);
    assert!(!a.is_unique());
}

