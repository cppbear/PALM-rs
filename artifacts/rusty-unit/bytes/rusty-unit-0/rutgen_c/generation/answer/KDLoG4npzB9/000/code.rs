// Answer 0

#[test]
fn test_split_off_must_use() {
    use crate::bytes::BytesMut;

    let mut b1 = BytesMut::from("hello world");
    let result = b1.split_off(6);
    assert_eq!(result, BytesMut::from("world"));
    assert_eq!(b1, BytesMut::from("hello "));
}

