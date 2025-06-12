// Answer 0

#[test]
fn test_split_to_full_split() {
    let mut a = Bytes::from(&b"hello"[..]);
    let b = a.split_to(5);
    
    assert_eq!(&a[..], b"");
    assert_eq!(&b[..], b"hello");
}

#[test]
fn test_split_to_partial_split() {
    let mut a = Bytes::from(&b"hello world"[..]);
    let b = a.split_to(5);
    
    assert_eq!(&a[..], b" world");
    assert_eq!(&b[..], b"hello");
}

#[test]
fn test_split_to_empty_input() {
    let mut a = Bytes::from(&b""[..]);
    let b = a.split_to(0);
    
    assert_eq!(&a[..], b"");
    assert_eq!(&b[..], b"");
}

#[test]
#[should_panic]
fn test_split_to_out_of_bounds() {
    let mut a = Bytes::from(&b"hello"[..]);
    let _ = a.split_to(6);  // This should panic
}

#[test]
fn test_split_to_zero_split() {
    let mut a = Bytes::from(&b"hello"[..]);
    let b = a.split_to(0);
    
    assert_eq!(&a[..], b"hello");
    assert_eq!(&b[..], b"");
}

