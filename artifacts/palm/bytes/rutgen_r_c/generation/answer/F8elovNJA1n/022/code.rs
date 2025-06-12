// Answer 0

#[test]
fn test_slice_excluded_start_bound() {
    let data = b"hello world";
    let bytes = Bytes::from_static(data);
    let result = bytes.slice(1..4);
    assert_eq!(&result[..], b"ell");
}

#[test]
fn test_slice_included_end_bound() {
    let data = b"hello world";
    let bytes = Bytes::from_static(data);
    let result = bytes.slice(0..5);
    assert_eq!(&result[..], b"hello");
}

#[test]
fn test_slice_empty() {
    let data = b"hello world";
    let bytes = Bytes::from_static(data);
    let result = bytes.slice(5..5);
    assert!(result.is_empty());
}

#[test]
#[should_panic(expected = "range start must not be greater than end")]
fn test_slice_panic_start_greater_than_end() {
    let data = b"hello world";
    let bytes = Bytes::from_static(data);
    let _ = bytes.slice(3..1);
}

#[test]
#[should_panic(expected = "range end out of bounds")]
fn test_slice_panic_end_out_of_bounds() {
    let data = b"hello world";
    let bytes = Bytes::from_static(data);
    let _ = bytes.slice(0..12);
}

#[test]
#[should_panic(expected = "range end out of bounds")]
fn test_slice_panic_start_exceeds_length() {
    let data = b"hello world";
    let bytes = Bytes::from_static(data);
    let _ = bytes.slice(11..12);
}

#[test]
fn test_slice_partial_overlap() {
    let data = b"hello world";
    let bytes = Bytes::from_static(data);
    let result = bytes.slice(6..11);
    assert_eq!(&result[..], b"world");
}

#[test]
fn test_slice_full_range() {
    let data = b"hello world";
    let bytes = Bytes::from_static(data);
    let result = bytes.slice(0..11);
    assert_eq!(&result[..], b"hello world");
}

