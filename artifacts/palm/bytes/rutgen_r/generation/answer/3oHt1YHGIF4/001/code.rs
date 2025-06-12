// Answer 0

#[test]
fn test_bytes_into_iter() {
    // Test with a static byte array
    let buf_static = Bytes::from_static(b"abc");
    let mut iter_static = buf_static.into_iter();
    
    assert_eq!(iter_static.next(), Some(b'a'));
    assert_eq!(iter_static.next(), Some(b'b'));
    assert_eq!(iter_static.next(), Some(b'c'));
    assert_eq!(iter_static.next(), None);

    // Test with an empty buffer
    let buf_empty = Bytes::from_static(b"");
    let mut iter_empty = buf_empty.into_iter();

    assert_eq!(iter_empty.next(), None);

    // Test with a longer byte array
    let buf_long = Bytes::from_static(b"Hello, world!");
    let mut iter_long = buf_long.into_iter();

    assert_eq!(iter_long.next(), Some(b'H'));
    assert_eq!(iter_long.next(), Some(b'e'));
    assert_eq!(iter_long.next(), Some(b'l'));
    assert_eq!(iter_long.next(), Some(b'l'));
    assert_eq!(iter_long.next(), Some(b'o'));
    assert_eq!(iter_long.next(), Some(b','));
    assert_eq!(iter_long.next(), Some(b' '));
    assert_eq!(iter_long.next(), Some(b'w'));
    assert_eq!(iter_long.next(), Some(b'o'));
    assert_eq!(iter_long.next(), Some(b'r'));
    assert_eq!(iter_long.next(), Some(b'l'));
    assert_eq!(iter_long.next(), Some(b'd'));
    assert_eq!(iter_long.next(), Some(b'!'));
    assert_eq!(iter_long.next(), None);
}

