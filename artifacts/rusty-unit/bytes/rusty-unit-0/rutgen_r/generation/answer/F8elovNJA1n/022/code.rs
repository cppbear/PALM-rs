// Answer 0

#[test]
fn test_slice_with_excluded_start_bound() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(2..5);
    
    assert_eq!(&b[..], b"llo");
}

#[test]
fn test_slice_with_included_end_bound() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(2..=5);
    
    assert_eq!(&b[..], b"llo ");
}

#[test]
fn test_slice_with_unbounded_start_and_included_end() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(..=5);
    
    assert_eq!(&b[..], b"hello ");
}

#[test]
fn test_slice_with_included_start_and_unbounded_end() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(6..);
    
    assert_eq!(&b[..], b"world");
}

#[test]
fn test_slice_with_equal_bounds() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(5..5);
    
    assert_eq!(&b[..], b"");
}

#[test]
#[should_panic(expected = "range start must not be greater than end")]
fn test_slice_with_invalid_bounds_start_greater_than_end() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    a.slice(5..3);
}

#[test]
#[should_panic(expected = "range end out of bounds")]
fn test_slice_with_end_out_of_bounds() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    a.slice(0..12);
}

