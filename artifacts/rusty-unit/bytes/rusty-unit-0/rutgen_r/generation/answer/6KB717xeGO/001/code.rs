// Answer 0

#[test]
fn test_set_limit_zero() {
    struct MockBufMut {
        limit: usize,
    }

    impl MockBufMut {
        fn new() -> Self {
            Self { limit: 0 }
        }

        fn set_limit(&mut self, lim: usize) {
            self.limit = lim;
        }
    }

    let mut buf = MockBufMut::new();
    buf.set_limit(0);
    assert_eq!(buf.limit, 0);
}

#[test]
fn test_set_limit_non_zero() {
    struct MockBufMut {
        limit: usize,
    }

    impl MockBufMut {
        fn new() -> Self {
            Self { limit: 0 }
        }

        fn set_limit(&mut self, lim: usize) {
            self.limit = lim;
        }
    }

    let mut buf = MockBufMut::new();
    buf.set_limit(123);
    assert_eq!(buf.limit, 123);
}

#[test]
fn test_set_limit_large_value() {
    struct MockBufMut {
        limit: usize,
    }

    impl MockBufMut {
        fn new() -> Self {
            Self { limit: 0 }
        }

        fn set_limit(&mut self, lim: usize) {
            self.limit = lim;
        }
    }

    let mut buf = MockBufMut::new();
    buf.set_limit(usize::MAX);
    assert_eq!(buf.limit, usize::MAX);
}

#[should_panic]
#[test]
fn test_set_limit_negative() {
    struct MockBufMut {
        limit: usize,
    }

    impl MockBufMut {
        fn new() -> Self {
            Self { limit: 0 }
        }

        fn set_limit(&mut self, lim: isize) {
            assert!(lim >= 0, "Limit must be non-negative");
            self.limit = lim as usize;
        }
    }

    let mut buf = MockBufMut::new();
    buf.set_limit(-1); // Here, we are forcing a panic
}

