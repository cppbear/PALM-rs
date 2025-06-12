// Answer 0

#[test]
fn test_slice_unbounded() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(..);
    assert_eq!(b.len(), a.len());
    assert_eq!(unsafe { core::slice::from_raw_parts(b.ptr, b.len) }, b"hello world");
}

#[test]
#[should_panic(expected = "range end out of bounds: 12 <= 11")]
fn test_slice_end_out_of_bounds() {
    let a = Bytes::from_static(b"hello world");
    let _ = a.slice(0..12);
}

#[test]
#[should_panic(expected = "range start must not be greater than end: 5 <= 5")]
fn test_slice_begin_equals_end() {
    let a = Bytes::from_static(b"hello world");
    let _ = a.slice(5..5);
}

#[test]
fn test_slice_empty() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(5..5);
    assert!(b.is_empty());
}

#[test]
#[should_panic(expected = "range start must not be greater than end: 6 <= 5")]
fn test_slice_invalid_range_start_greater_than_end() {
    let a = Bytes::from_static(b"hello");
    let _ = a.slice(6..5);
}

