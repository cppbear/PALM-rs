// Answer 0

#[test]
fn test_split_off_must_use() {
    use crate::Bytes;

    let b1 = Bytes::from("hello world");
    // We must use the result of split_off to avoid compile failure
    let result = b1.split_off(6);
    assert_eq!(result.as_ref(), b"world");
}

#[test]
#[should_panic]
fn test_split_off_panic_too_large() {
    use crate::Bytes;

    let b1 = Bytes::from("hello");
    // This should panic because the split off index is out of bounds
    let _result = b1.split_off(10);
}

#[test]
fn test_split_off_empty() {
    use crate::Bytes;

    let b1 = Bytes::from("");
    // Splitting off at index 0 results in two empty Bytes
    let result = b1.split_off(0);
    assert!(result.is_empty());
}

#[test]
fn test_split_off_full_length() {
    use crate::Bytes;

    let b1 = Bytes::from("test");
    // Splitting off at the full length results in an empty Bytes
    let result = b1.split_off(4);
    assert!(result.is_empty());
}

