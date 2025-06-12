// Answer 0

#[test]
fn test_slice_included_excluded_valid() {
    let a = Bytes::from_static(b"hello");
    let b = a.slice(1..4);
}

#[test]
#[should_panic]
fn test_slice_included_included_invalid() {
    let a = Bytes::from_static(b"hello");
    let b = a.slice(1..=3);
}

#[test]
#[should_panic]
fn test_slice_excluded_excluded_invalid() {
    let a = Bytes::from_static(b"hello");
    let b = a.slice(0..3);
}

#[test]
#[should_panic]
fn test_slice_included_excluded_out_of_bounds() {
    let a = Bytes::from_static(b"hello");
    let b = a.slice(0..5);
}

#[test]
#[should_panic]
fn test_slice_not_satisfy_begin_end() {
    let a = Bytes::from_static(b"hello");
    let b = a.slice(3..2);
}

#[test]
fn test_slice_empty_range() {
    let a = Bytes::from_static(b"hello");
    let b = a.slice(3..3);
}

#[test]
fn test_slice_full_range() {
    let a = Bytes::from_static(b"hello");
    let b = a.slice(0..5);
}

