// Answer 0

#[test]
fn test_is_unique_static_slice() {
    let bytes = Bytes::from_static(b"static slice");
    assert!(!bytes.is_unique());
}

#[test]
fn test_is_unique_empty() {
    let bytes = Bytes::new();
    assert!(bytes.is_unique());
}

#[test]
fn test_is_unique_owner() {
    let bytes = Bytes::from_owner(vec![1, 2, 3]);
    assert!(bytes.is_unique());
}

#[test]
fn test_is_unique_after_clone() {
    let bytes = Bytes::from(vec![1, 2, 3]);
    assert!(bytes.is_unique());
    let _clone = bytes.clone();
    assert!(!bytes.is_unique());
}

#[test]
#[should_panic(expected = "out of range")]
fn test_slice_out_of_range_start() {
    let bytes = Bytes::from(vec![1, 2, 3]);
    bytes.slice(5..10);
}

#[test]
#[should_panic(expected = "out of range")]
fn test_slice_out_of_range_end() {
    let bytes = Bytes::from(vec![1, 2, 3]);
    bytes.slice(1..5);
}

#[test]
fn test_slice_valid_range() {
    let bytes = Bytes::from(vec![1, 2, 3, 4, 5]);
    let sliced = bytes.slice(1..4);
    assert_eq!(sliced.len(), 3);
}

#[test]
fn test_split_off_empty() {
    let mut bytes = Bytes::from(vec![1, 2, 3]);
    let split = bytes.split_off(0);
    assert_eq!(bytes.len(), 0);
    assert_eq!(split.len(), 3);
}

#[test]
fn test_split_off_at_end() {
    let mut bytes = Bytes::from(vec![1, 2, 3]);
    let split = bytes.split_off(3);
    assert_eq!(bytes.len(), 0);
    assert_eq!(split.len(), 3);
}

#[test]
#[should_panic(expected = "split_off out of bounds")]
fn test_split_off_out_of_bounds() {
    let mut bytes = Bytes::from(vec![1, 2, 3]);
    bytes.split_off(4);
}

