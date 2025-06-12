// Answer 0

#[test]
fn test_slice_valid_range() {
    let bytes = Bytes::from_static(b"hello world");
    let slice_bytes = bytes.slice(2..5);
    assert_eq!(&slice_bytes[..], b"llo");
}

#[test]
fn test_slice_empty_range() {
    let bytes = Bytes::from_static(b"hello world");
    let slice_bytes = bytes.slice(5..5);
    assert!(slice_bytes.is_empty());
}

#[test]
#[should_panic(expected = "range start must not be greater than end")]
fn test_slice_range_start_greater() {
    let bytes = Bytes::from_static(b"hello world");
    bytes.slice(5..3);
}

#[test]
#[should_panic(expected = "range end out of bounds")]
fn test_slice_range_end_out_of_bounds() {
    let bytes = Bytes::from_static(b"hello world");
    bytes.slice(0..12);
}

#[test]
fn test_slice_full_range() {
    let bytes = Bytes::from_static(b"hello world");
    let slice_bytes = bytes.slice(0..11);
    assert_eq!(&slice_bytes[..], b"hello world");
}

#[test]
fn test_slice_unbounded_upper() {
    let bytes = Bytes::from_static(b"hello world");
    let slice_bytes = bytes.slice(0..);
    assert_eq!(&slice_bytes[..], b"hello world");
}

#[test]
fn test_slice_unbounded_lower() {
    let bytes = Bytes::from_static(b"hello world");
    let slice_bytes = bytes.slice(..5);
    assert_eq!(&slice_bytes[..], b"hello");
}

