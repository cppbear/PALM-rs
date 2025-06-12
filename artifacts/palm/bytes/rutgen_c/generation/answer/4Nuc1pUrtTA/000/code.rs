// Answer 0

#[test]
fn test_try_into_mut_success() {
    let bytes = Bytes::from_owner(vec![1, 2, 3]);
    let result = bytes.try_into_mut();
    assert!(result.is_ok());
    let bytes_mut = result.unwrap();
    assert_eq!(bytes_mut.len(), 3);
}

#[test]
fn test_try_into_mut_failure() {
    let bytes = Bytes::from_static(b"hello");
    let result = bytes.try_into_mut();
    assert!(result.is_err());
}

#[test]
fn test_try_into_mut_with_unique_bytes() {
    let bytes = Bytes::copy_from_slice(b"unique");
    let result = bytes.try_into_mut();
    assert!(result.is_ok());
    let bytes_mut = result.unwrap();
    assert_eq!(bytes_mut.len(), 6);
}

#[test]
#[should_panic]
fn test_try_into_mut_empty_bytes() {
    let empty_bytes = Bytes::new();
    let _ = empty_bytes.try_into_mut();
}

