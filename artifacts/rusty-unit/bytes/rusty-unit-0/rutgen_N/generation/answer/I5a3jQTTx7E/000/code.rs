// Answer 0

#[test]
#[should_panic]
fn test_split_must_use() {
    use bytes::BytesMut;
    #[deny(unused_must_use)]
    {
        let mut b1 = BytesMut::from("hello world");
        b1.split();
    }
}

