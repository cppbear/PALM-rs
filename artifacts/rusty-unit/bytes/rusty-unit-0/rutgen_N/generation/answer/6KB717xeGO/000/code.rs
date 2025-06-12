// Answer 0

#[derive(Default)]
struct TestBufMut {
    buf: Vec<u8>,
}

impl TestBufMut {
    fn available_bytes(&self) -> usize {
        self.buf.len()
    }
}

struct Buffer {
    limit: usize,
    inner: TestBufMut,
}

impl Buffer {
    pub fn new(inner: TestBufMut) -> Self {
        Self { limit: 0, inner }
    }

    pub fn set_limit(&mut self, lim: usize) {
        self.limit = lim;
    }
}

#[test]
fn test_set_limit_with_zero() {
    let inner_buf = TestBufMut::default(); // empty buffer
    let mut buffer = Buffer::new(inner_buf);

    buffer.set_limit(0);
    assert_eq!(buffer.limit, 0);
}

#[test]
fn test_set_limit_with_negative() {
    let inner_buf = TestBufMut::default(); // empty buffer
    let mut buffer = Buffer::new(inner_buf);

    buffer.set_limit(10);
    assert_eq!(buffer.limit, 10);
}

#[test]
fn test_set_limit_exceeding_available_bytes() {
    let inner_buf = TestBufMut { buf: vec![1, 2, 3] }; // available bytes is 3
    let mut buffer = Buffer::new(inner_buf);

    buffer.set_limit(5);
    assert_eq!(buffer.limit, 5);
}

#[test]
fn test_set_limit_exactly_available_bytes() {
    let inner_buf = TestBufMut { buf: vec![1, 2] }; // available bytes is 2
    let mut buffer = Buffer::new(inner_buf);

    buffer.set_limit(2);
    assert_eq!(buffer.limit, 2);
}

