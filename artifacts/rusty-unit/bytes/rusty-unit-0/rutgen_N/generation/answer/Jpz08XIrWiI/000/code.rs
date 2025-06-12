// Answer 0

#[test]
fn test_remaining_with_inner_remaining_greater_than_limit() {
    struct Inner {
        remaining_count: usize,
    }

    impl Inner {
        fn remaining(&self) -> usize {
            self.remaining_count
        }
    }

    struct LimitedBuffer {
        inner: Inner,
        limit: usize,
    }

    impl LimitedBuffer {
        fn remaining(&self) -> usize {
            std::cmp::min(self.inner.remaining(), self.limit)
        }
    }

    let inner = Inner { remaining_count: 10 };
    let buffer = LimitedBuffer { inner, limit: 5 };
    assert_eq!(buffer.remaining(), 5);
}

#[test]
fn test_remaining_with_inner_remaining_equal_to_limit() {
    struct Inner {
        remaining_count: usize,
    }

    impl Inner {
        fn remaining(&self) -> usize {
            self.remaining_count
        }
    }

    struct LimitedBuffer {
        inner: Inner,
        limit: usize,
    }

    impl LimitedBuffer {
        fn remaining(&self) -> usize {
            std::cmp::min(self.inner.remaining(), self.limit)
        }
    }

    let inner = Inner { remaining_count: 5 };
    let buffer = LimitedBuffer { inner, limit: 5 };
    assert_eq!(buffer.remaining(), 5);
}

#[test]
fn test_remaining_with_inner_remaining_less_than_limit() {
    struct Inner {
        remaining_count: usize,
    }

    impl Inner {
        fn remaining(&self) -> usize {
            self.remaining_count
        }
    }

    struct LimitedBuffer {
        inner: Inner,
        limit: usize,
    }

    impl LimitedBuffer {
        fn remaining(&self) -> usize {
            std::cmp::min(self.inner.remaining(), self.limit)
        }
    }

    let inner = Inner { remaining_count: 3 };
    let buffer = LimitedBuffer { inner, limit: 5 };
    assert_eq!(buffer.remaining(), 3);
}

#[test]
fn test_remaining_with_zero_limit() {
    struct Inner {
        remaining_count: usize,
    }

    impl Inner {
        fn remaining(&self) -> usize {
            self.remaining_count
        }
    }

    struct LimitedBuffer {
        inner: Inner,
        limit: usize,
    }

    impl LimitedBuffer {
        fn remaining(&self) -> usize {
            std::cmp::min(self.inner.remaining(), self.limit)
        }
    }

    let inner = Inner { remaining_count: 10 };
    let buffer = LimitedBuffer { inner, limit: 0 };
    assert_eq!(buffer.remaining(), 0);
}

