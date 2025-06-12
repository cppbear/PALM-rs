// Answer 0

#[test]
fn test_is_unique_with_vector() {
    use bytes::Bytes;

    let a = Bytes::from(vec![1, 2, 3]);
    assert!(a.is_unique());

    let b = a.clone();
    assert!(!a.is_unique());
}

#[test]
fn test_is_unique_with_static_slice() {
    use bytes::Bytes;

    let a = Bytes::from_static(&[1, 2, 3]);
    assert!(!a.is_unique());
}

#[test]
fn test_is_unique_with_owner() {
    use bytes::Bytes;

    let a = Bytes::from_owner(vec![1, 2, 3]);
    assert!(!a.is_unique());

    let b = a.clone();
    assert!(!a.is_unique());
}

