// Answer 0

#[test]
fn test_split_off_zero() {
    let mut a = Bytes::from_static(b"hello world");
    let b = a.split_off(0);
    
    assert_eq!(&a[..], b"");
    assert_eq!(&b[..], b"hello world");
}

#[test]
#[should_panic(expected = "split_off out of bounds: 5 <= 5")]
fn test_split_off_out_of_bounds() {
    let mut a = Bytes::from_static(b"hello");
    let _ = a.split_off(5);
}

#[test]
fn test_split_off_non_zero() {
    let mut a = Bytes::from_static(b"hello world");
    let b = a.split_off(5);
    
    assert_eq!(&a[..], b"hello");
    assert_eq!(&b[..], b" world");
}

