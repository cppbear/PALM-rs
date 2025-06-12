// Answer 0

#[test]
fn test_get_ref_return_capacity() {
    struct TestBufMut {
        buf: Vec<u8>,
    }

    impl TestBufMut {
        fn new(capacity: usize) -> Self {
            TestBufMut {
                buf: Vec::with_capacity(capacity),
            }
        }

        fn writer(self) -> &Self {
            &self
        }
    }

    let buf = TestBufMut::new(1024).writer();
    assert_eq!(1024, buf.buf.capacity());
}

#[test]
fn test_get_ref_empty_capacity() {
    struct TestBufMut {
        buf: Vec<u8>,
    }

    impl TestBufMut {
        fn new() -> Self {
            TestBufMut {
                buf: Vec::new(),
            }
        }

        fn writer(self) -> &Self {
            &self
        }
    }

    let buf = TestBufMut::new().writer();
    assert_eq!(0, buf.buf.capacity());
}

#[test]
#[should_panic]
fn test_get_ref_panic_on_empty_vec() {
    struct TestBufMut {
        buf: Vec<u8>,
    }

    impl TestBufMut {
        fn new() -> Self {
            TestBufMut {
                buf: Vec::new(),
            }
        }

        fn writer(self) -> &Self {
            &self
        }
    }

    let buf = TestBufMut::new().writer();
    let _ = buf.buf[0]; // Accessing element should panic since the vec is empty.
}

