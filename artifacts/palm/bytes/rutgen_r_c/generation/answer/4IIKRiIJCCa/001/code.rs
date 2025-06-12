// Answer 0

#[test]
fn test_assert_trait_object_valid_buf_mut() {
    struct TestBufMut {
        data: Vec<u8>,
        offset: usize,
    }

    impl TestBufMut {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size],
                offset: 0,
            }
        }
    }

    let buf: &dyn BufMut = &TestBufMut::new(10);
    _assert_trait_object(buf);
}

#[should_panic]
#[test]
fn test_assert_trait_object_invalid_buf_mut() {
    struct OtherTestBufMut;

    let buf: &dyn BufMut = &OtherTestBufMut;
    _assert_trait_object(buf);
}

