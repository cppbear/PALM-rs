// Answer 0

#[test]
fn test_get_ref() {
    struct TestBuf {
        data: &'static [u8],
    }

    impl TestBuf {
        fn new(data: &'static [u8]) -> Self {
            TestBuf { data }
        }
    }

    struct TestReader<B> {
        buf: B,
    }

    impl<B> TestReader<B> {
        fn new(buf: B) -> Self {
            TestReader { buf }
        }

        fn get_ref(&self) -> &B {
            &self.buf
        }
    }

    let buf = TestReader::new(TestBuf::new(b"hello world"));
    assert_eq!(buf.get_ref().data, b"hello world");
}

