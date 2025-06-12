// Answer 0

#[test]
fn test_split_off_must_use() {
    use bytes::Bytes;
    #[deny(unused_must_use)]
    {
        let mut b1 = Bytes::from("hello world");
        let _ = b1.split_off(6);
    }
}

