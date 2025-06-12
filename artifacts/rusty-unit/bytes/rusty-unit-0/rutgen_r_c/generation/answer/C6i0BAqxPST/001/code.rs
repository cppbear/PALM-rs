// Answer 0

#[test]
fn test_unsplit_when_self_is_empty() {
    let mut buf = BytesMut::new();
    let other = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);

    buf.unsplit(other);

    assert_eq!(buf.len(), 5);
    assert_eq!(buf.as_slice(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_unsplit_when_self_is_empty_and_other_is_empty() {
    let mut buf = BytesMut::new();
    let other = BytesMut::new();

    buf.unsplit(other);

    assert_eq!(buf.len(), 0);
    assert!(buf.is_empty());
}

#[test]
fn test_unsplit_duplicates_reference_when_other_was_split_off() {
    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"abcdef");

    let split = buf.split_off(3);
    assert_eq!(b"abc", &buf[..]);
    assert_eq!(b"def", &split[..]);

    buf.unsplit(split);
    assert_eq!(b"abcdef", &buf[..]);
    assert_eq!(buf.len(), 6);
}

#[test]
fn test_unsplit_when_self_not_empty_and_other_empty() {
    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(b"abc");

    let other = BytesMut::new();

    buf.unsplit(other);

    assert_eq!(buf.len(), 3);
    assert_eq!(buf.as_slice(), b"abc");
}

#[test]
fn test_unsplit_when_self_not_empty_and_other_more_than_split() {
    let mut buf = BytesMut::new();
    buf.extend_from_slice(b"abcd");

    let other = BytesMut::from_vec(vec![e, f, g, h]);
    
    buf.unsplit(other);
    
    assert_eq!(buf.len(), 8);
    assert_eq!(buf.as_slice(), b"abcdefgh");
}

