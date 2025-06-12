// Answer 0

#[test]
fn test_slice_valid_range() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(2..5);

    assert_eq!(&b[..], b"llo");
}

#[test]
fn test_slice_empty_range() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(5..5);

    assert_eq!(&b[..], b"");
}

#[test]
#[should_panic]
fn test_slice_start_greater_than_end() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let _ = a.slice(5..2);
}

#[test]
#[should_panic]
fn test_slice_end_out_of_bounds() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let _ = a.slice(0..12);
}

#[test]
#[should_panic]
fn test_slice_start_out_of_bounds() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let _ = a.slice(12..13);
}

