// Answer 0

#[test]
fn test_slice_unbounded() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(..);
}

#[test]
#[should_panic]
fn test_slice_begin_equals_end() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(5..5);
}

#[test]
#[should_panic]
fn test_slice_end_out_of_bounds() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(0..15);
}

#[test]
#[should_panic]
fn test_slice_invalid_range_begins_greater_than_end() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(5..3);
}

