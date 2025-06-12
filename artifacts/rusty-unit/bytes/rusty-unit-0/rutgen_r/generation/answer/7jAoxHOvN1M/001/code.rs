// Answer 0

#[test]
fn test_buf_limit_with_non_zero_limit() {
    struct TestBufMut {
        limit: usize,
    }
    
    impl TestBufMut {
        fn new(limit: usize) -> Self {
            Self { limit }
        }

        fn limit(&self) -> usize {
            self.limit
        }
    }

    let buf = TestBufMut::new(100);
    assert_eq!(buf.limit(), 100);
}

#[test]
fn test_buf_limit_with_zero_limit() {
    struct TestBufMut {
        limit: usize,
    }
    
    impl TestBufMut {
        fn new(limit: usize) -> Self {
            Self { limit }
        }

        fn limit(&self) -> usize {
            self.limit
        }
    }

    let buf = TestBufMut::new(0);
    assert_eq!(buf.limit(), 0);
}

