// Answer 0

#[test]
fn test_slice_range_included_valid_case() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(2..5);
}

#[test]
fn test_slice_range_empty_case() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(5..5);
}

#[test]
fn test_slice_range_full_case() {
    let a = Bytes::from_static(b"hello");
    let b = a.slice(0..5);
}

#[should_panic]
fn test_slice_range_panic_begin_greater_than_end() {
    let a = Bytes::from_static(b"hello world");
    let _ = a.slice(5..2);
}

#[should_panic]
fn test_slice_range_panic_end_out_of_bounds() {
    let a = Bytes::from_static(b"hello");
    let _ = a.slice(0..6);
}

#[should_panic]
fn test_slice_range_panic_begin_out_of_bounds() {
    let a = Bytes::from_static(b"hello");
    let _ = a.slice(1..10);
}

#[test]
fn test_slice_range_included_at_bounds() {
    let a = Bytes::from_static(b"hello");
    let b = a.slice(0..0);
    let c = a.slice(5..5);
}

