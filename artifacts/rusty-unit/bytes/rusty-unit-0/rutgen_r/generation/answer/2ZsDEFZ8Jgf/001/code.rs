// Answer 0

#[test]
fn test_limit_with_max_capacity() {
    struct TestBuf {
        limit: usize,
    }

    impl TestBuf {
        fn new(limit: usize) -> Self {
            TestBuf { limit }
        }

        fn limit(&self) -> usize {
            self.limit
        }
    }

    let buf = TestBuf::new(10);
    assert_eq!(10, buf.limit());
}

#[test]
fn test_limit_with_zero_capacity() {
    struct TestBuf {
        limit: usize,
    }

    impl TestBuf {
        fn new(limit: usize) -> Self {
            TestBuf { limit }
        }

        fn limit(&self) -> usize {
            self.limit
        }
    }

    let buf = TestBuf::new(0);
    assert_eq!(0, buf.limit());
}

#[test]
fn test_limit_with_one_capacity() {
    struct TestBuf {
        limit: usize,
    }

    impl TestBuf {
        fn new(limit: usize) -> Self {
            TestBuf { limit }
        }

        fn limit(&self) -> usize {
            self.limit
        }
    }

    let buf = TestBuf::new(1);
    assert_eq!(1, buf.limit());
}

