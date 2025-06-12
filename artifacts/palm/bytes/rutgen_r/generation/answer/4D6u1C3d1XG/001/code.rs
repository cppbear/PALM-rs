// Answer 0

#[test]
fn test_split_to_at_len() {
    use bytes::BytesMut;

    let mut buffer = BytesMut::from(&b"hello"[..]);
    let split_buffer = buffer.split_to(buffer.len());

    assert_eq!(&buffer[..], b"");
    assert_eq!(&split_buffer[..], b"hello");
}

#[test]
fn test_split_to_at_zero() {
    use bytes::BytesMut;

    let mut buffer = BytesMut::from(&b"test"[..]);
    let split_buffer = buffer.split_to(0);

    assert_eq!(&buffer[..], b"test");
    assert_eq!(&split_buffer[..], b"");
}

#[should_panic(expected = "split_to out of bounds")]
fn test_split_to_out_of_bounds() {
    use bytes::BytesMut;

    let mut buffer = BytesMut::from(&b"abc"[..]);
    let _ = buffer.split_to(4); // This should panic since at > len
}

#[test]
fn test_split_to_with_large_buffer() {
    use bytes::BytesMut;

    let mut buffer = BytesMut::from(&b"abcdefg"[..]);
    let split_buffer = buffer.split_to(3);

    assert_eq!(&buffer[..], b"defg");
    assert_eq!(&split_buffer[..], b"abc");
}

