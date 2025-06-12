// Answer 0

#[test]
fn test_remaining_mut_with_limit_greater_than_remaining() {
    struct Inner {
        remaining: usize,
    }

    impl Inner {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
    }

    struct LimitedBuffer {
        inner: Inner,
        limit: usize,
    }

    impl LimitedBuffer {
        fn remaining_mut(&self) -> usize {
            std::cmp::min(self.inner.remaining_mut(), self.limit)
        }
    }

    let buffer = LimitedBuffer {
        inner: Inner { remaining: 5 },
        limit: 10,
    };

    assert_eq!(buffer.remaining_mut(), 5);
}

#[test]
fn test_remaining_mut_with_limit_less_than_remaining() {
    struct Inner {
        remaining: usize,
    }

    impl Inner {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
    }

    struct LimitedBuffer {
        inner: Inner,
        limit: usize,
    }

    impl LimitedBuffer {
        fn remaining_mut(&self) -> usize {
            std::cmp::min(self.inner.remaining_mut(), self.limit)
        }
    }

    let buffer = LimitedBuffer {
        inner: Inner { remaining: 15 },
        limit: 10,
    };

    assert_eq!(buffer.remaining_mut(), 10);
}

#[test]
fn test_remaining_mut_with_equal_remaining_and_limit() {
    struct Inner {
        remaining: usize,
    }

    impl Inner {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
    }

    struct LimitedBuffer {
        inner: Inner,
        limit: usize,
    }

    impl LimitedBuffer {
        fn remaining_mut(&self) -> usize {
            std::cmp::min(self.inner.remaining_mut(), self.limit)
        }
    }

    let buffer = LimitedBuffer {
        inner: Inner { remaining: 10 },
        limit: 10,
    };

    assert_eq!(buffer.remaining_mut(), 10);
}

#[test]
fn test_remaining_mut_with_zero_remaining() {
    struct Inner {
        remaining: usize,
    }

    impl Inner {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
    }

    struct LimitedBuffer {
        inner: Inner,
        limit: usize,
    }

    impl LimitedBuffer {
        fn remaining_mut(&self) -> usize {
            std::cmp::min(self.inner.remaining_mut(), self.limit)
        }
    }

    let buffer = LimitedBuffer {
        inner: Inner { remaining: 0 },
        limit: 10,
    };

    assert_eq!(buffer.remaining_mut(), 0);
}

#[test]
fn test_remaining_mut_with_zero_limit() {
    struct Inner {
        remaining: usize,
    }

    impl Inner {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
    }

    struct LimitedBuffer {
        inner: Inner,
        limit: usize,
    }

    impl LimitedBuffer {
        fn remaining_mut(&self) -> usize {
            std::cmp::min(self.inner.remaining_mut(), self.limit)
        }
    }

    let buffer = LimitedBuffer {
        inner: Inner { remaining: 5 },
        limit: 0,
    };

    assert_eq!(buffer.remaining_mut(), 0);
}

