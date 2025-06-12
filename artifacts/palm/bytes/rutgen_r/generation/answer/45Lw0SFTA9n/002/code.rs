// Answer 0

#[test]
fn split_to_middle() {
    let mut a = Bytes::from(&b"hello world"[..]);
    let b = a.split_to(5);
    
    assert_eq!(&a[..], b" world");
    assert_eq!(&b[..], b"hello");
}

#[test]
fn split_to_full_length() {
    let mut a = Bytes::from(&b"hello"[..]);
    let b = a.split_to(4);
    
    assert_eq!(&a[..], b"o");
    assert_eq!(&b[..], b"hell");
}

#[test]
fn split_to_one() {
    let mut a = Bytes::from(&b"hello"[..]);
    let b = a.split_to(1);
    
    assert_eq!(&a[..], b"ello");
    assert_eq!(&b[..], b"h");
}

#[test]
#[should_panic]
fn split_to_panic_at_greater_length() {
    let mut a = Bytes::from(&b"hello"[..]);
    a.split_to(6); // This should panic as 6 > 5
}

#[test]
#[should_panic]
fn split_to_panic_at_zero_length() {
    let mut a = Bytes::from(&b"hello"[..]);
    a.split_to(0); // This should panic as at == 0
}

