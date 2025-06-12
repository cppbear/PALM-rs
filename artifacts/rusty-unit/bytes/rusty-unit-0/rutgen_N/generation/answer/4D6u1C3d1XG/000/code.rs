// Answer 0

#[test]
fn test_split_to_within_bounds() {
    use bytes::BytesMut;

    let mut buffer = BytesMut::from(&b"hello world"[..]);
    let split_buffer = buffer.split_to(5);

    assert_eq!(&buffer[..], b" world");
    assert_eq!(&split_buffer[..], b"hello");
}

#[test]
fn test_split_to_edge_case() {
    use bytes::BytesMut;

    let mut buffer = BytesMut::from(&b"hello"[..]);
    let split_buffer = buffer.split_to(0);

    assert_eq!(&buffer[..], b"hello");
    assert_eq!(&split_buffer[..], b"");
}

#[should_panic]
fn test_split_to_out_of_bounds() {
    use bytes::BytesMut;

    let mut buffer = BytesMut::from(&b"test"[..]);
    let _ = buffer.split_to(5);
}

