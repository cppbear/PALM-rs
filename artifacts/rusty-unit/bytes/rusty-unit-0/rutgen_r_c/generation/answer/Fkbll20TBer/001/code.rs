// Answer 0

#[test]
fn test_from_static_empty_slice() {
    static EMPTY: &'static [u8] = &[];
    let b = Bytes::from_static(EMPTY);
    assert_eq!(b.len(), 0);
    assert!(b.is_empty());
    assert_eq!(b.as_slice(), &[]);
}

#[test]
fn test_from_static_non_empty_slice() {
    static HELLO: &'static [u8] = b"hello";
    let b = Bytes::from_static(HELLO);
    assert_eq!(b.len(), HELLO.len());
    assert_eq!(b.as_slice(), HELLO);
}

#[test]
fn test_from_static_large_slice() {
    static LARGE: &'static [u8] = b"this is a large static byte slice used for testing";
    let b = Bytes::from_static(LARGE);
    assert_eq!(b.len(), LARGE.len());
    assert_eq!(b.as_slice(), LARGE);
}

#[should_panic(expected = "slice index out of bounds")]
#[test]
fn test_from_static_null_slice() {
    static NULL: &'static [u8] = unsafe { core::mem::transmute::<&[u8], &'static [u8]>(ptr::null()) };
    let _ = Bytes::from_static(NULL); // This should panic
}

