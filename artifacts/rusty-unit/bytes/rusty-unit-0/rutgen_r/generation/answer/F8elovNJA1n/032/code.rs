// Answer 0

#[test]
fn test_slice_included_included() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(0..5);
    
    assert_eq!(&b[..], b"hello");
}

#[test]
fn test_slice_included_excluded() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(0..5);
    
    assert_eq!(&b[..], b"hello");
}

#[test]
fn test_slice_excluded_included() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(0..10); 
    
    assert_eq!(&b[..], b"hello worl");
}

#[test]
#[should_panic(expected = "range start must not be greater than end")]
fn test_slice_begin_greater_than_end() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let _ = a.slice(5..3);
}

#[test]
#[should_panic(expected = "range end out of bounds")]
fn test_slice_end_out_of_bounds() {
    use bytes::Bytes;

    let a = Bytes::from(&b"hello world"[..]);
    let _ = a.slice(0..20);
}

