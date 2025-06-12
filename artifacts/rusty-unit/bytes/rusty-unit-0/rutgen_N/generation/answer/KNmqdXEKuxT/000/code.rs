// Answer 0

#[test]
fn test_last_ref() {
    struct TestBuf { b: &'static [u8] }

    impl TestBuf {
        fn new(data: &'static [u8]) -> Self {
            TestBuf { b: data }
        }
    }

    struct Chain<'a> {
        buf: &'a TestBuf,
    }

    impl<'a> Chain<'a> {
        fn new(buf: &'a TestBuf) -> Self {
            Chain { buf }
        }
        
        fn last_ref(&self) -> &'a [u8] {
            self.buf.b
        }
    }

    let buf1 = TestBuf::new(b"hello");
    let buf2 = TestBuf::new(b"world");
    let chain = Chain::new(&buf2);

    assert_eq!(chain.last_ref(), b"world");
}

