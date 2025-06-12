// Answer 0

#[test]
fn test_split_to_must_use() {
    use bytes::Bytes;
    #[deny(unused_must_use)]
    {
        let mut b1 = Bytes::from("hello world");
        let result = b1.split_to(6);
        assert_eq!(&result[..], b"hello ");
        assert_eq!(&b1[..], b"world");
    }
}

#[should_panic]
#[test]
fn test_split_to_panic_on_too_large_split() {
    use bytes::Bytes;
    #[deny(unused_must_use)]
    {
        let mut b1 = Bytes::from("hello");
        let _result = b1.split_to(10);
    }
}

