// Answer 0

#[derive(Default)]
struct InnerBuffer {
    remaining: usize,
}

impl InnerBuffer {
    fn remaining_mut(&self) -> usize {
        self.remaining
    }
}

struct LimitedBuffer {
    inner: InnerBuffer,
    limit: usize,
}

impl LimitedBuffer {
    fn remaining_mut(&self) -> usize {
        std::cmp::min(self.inner.remaining_mut(), self.limit)
    }
}

#[test]
fn test_remaining_mut_within_limit() {
    let inner = InnerBuffer { remaining: 10 };
    let buffer = LimitedBuffer { inner, limit: 15 };
    assert_eq!(buffer.remaining_mut(), 10);
}

#[test]
fn test_remaining_mut_exceeds_limit() {
    let inner = InnerBuffer { remaining: 20 };
    let buffer = LimitedBuffer { inner, limit: 15 };
    assert_eq!(buffer.remaining_mut(), 15);
}

#[test]
fn test_remaining_mut_zero() {
    let inner = InnerBuffer { remaining: 0 };
    let buffer = LimitedBuffer { inner, limit: 15 };
    assert_eq!(buffer.remaining_mut(), 0);
}

#[test]
fn test_remaining_mut_limit_zero() {
    let inner = InnerBuffer { remaining: 10 };
    let buffer = LimitedBuffer { inner, limit: 0 };
    assert_eq!(buffer.remaining_mut(), 0);
}

