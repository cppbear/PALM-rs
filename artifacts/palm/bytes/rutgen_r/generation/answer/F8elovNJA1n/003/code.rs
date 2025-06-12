// Answer 0

#[test]
fn test_slice_unbounded_both_bounds() {
    use bytes::Bytes;
    
    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(..);
    
    assert_eq!(&b[..], b"hello world");
}

#[test]
fn test_slice_begin_equal_end() {
    use bytes::Bytes;
    
    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(5..5);
    
    assert_eq!(&b[..], b"");
}

#[should_panic(expected = "range end out of bounds")]
fn test_slice_end_out_of_bounds() {
    use bytes::Bytes;
    
    let a = Bytes::from(&b"hello world"[..]);
    let _ = a.slice(0..15);
}

#[should_panic(expected = "range start must not be greater than end")]
fn test_slice_begin_greater_than_end() {
    use bytes::Bytes;
    
    let a = Bytes::from(&b"hello world"[..]);
    let _ = a.slice(6..5);
}

