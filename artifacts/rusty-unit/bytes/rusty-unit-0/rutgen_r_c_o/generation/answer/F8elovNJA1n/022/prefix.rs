// Answer 0

#[test]
fn test_slice_range_excluded_included() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(1..5);
}

#[test]
fn test_slice_range_included_excluded() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(0..4);
}

#[test]
fn test_slice_range_excluded_excluded() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(2..5);
}

#[test]
fn test_slice_range_half_included() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(2..9);
}

#[test]
fn test_slice_identical_range() {
    let a = Bytes::from_static(b"hello");
    let b = a.slice(0..0);
}

#[test]
fn test_slice_mid_range() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(6..11);
}

#[test]
fn test_slice_full_range() {
    let a = Bytes::from_static(b"hello world");
    let b = a.slice(0..11);
}

