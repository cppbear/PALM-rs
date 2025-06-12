// Answer 0

#[test]
fn test_slice_valid_range_included() {
    let bytes = Bytes::from_static(b"hello world");
    let sliced = bytes.slice(2..5);
    assert_eq!(&sliced.as_slice()[..], b"llo");
}

#[test]
fn test_slice_valid_range_unbounded() {
    let bytes = Bytes::from_static(b"hello world");
    let sliced = bytes.slice(..5);
    assert_eq!(&sliced.as_slice()[..], b"hello");
}

#[test]
#[should_panic(expected = "out of range")]
fn test_slice_invalid_range_start_greater_than_end() {
    let bytes = Bytes::from_static(b"hello world");
    let _ = bytes.slice(5..2);
}

#[test]
#[should_panic(expected = "range end out of bounds")]
fn test_slice_invalid_range_end_exceeds_length() {
    let bytes = Bytes::from_static(b"hello world");
    let _ = bytes.slice(0..15);
}

#[test]
#[should_panic(expected = "out of range")]
fn test_slice_invalid_range_start_exceeds_length() {
    let bytes = Bytes::from_static(b"hello world");
    let _ = bytes.slice(12..15);
}

#[test]
fn test_slice_empty_range() {
    let bytes = Bytes::from_static(b"hello world");
    let sliced = bytes.slice(5..5);
    assert!(sliced.is_empty());
}

