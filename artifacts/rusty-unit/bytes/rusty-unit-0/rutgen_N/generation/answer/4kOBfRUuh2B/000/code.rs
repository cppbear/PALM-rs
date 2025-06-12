// Answer 0

#[test]
fn test_get_mut() {
    struct DummyBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl DummyBuf {
        fn new(data: Vec<u8>) -> Self {
            DummyBuf { data, position: 0 }
        }

        fn advance(&mut self, count: usize) {
            self.position += count;
        }
    }

    struct TakeBuf<T> {
        inner: T,
    }

    impl<T> TakeBuf<T> {
        fn new(inner: T) -> Self {
            TakeBuf { inner }
        }

        pub fn get_mut(&mut self) -> &mut T {
            &mut self.inner
        }
    }

    let mut buf = TakeBuf::new(DummyBuf::new(b"hello world".to_vec()));
    let mut dst = vec![];

    buf.get_mut().advance(2);

    // Simulating `put` functionality for example purposes
    dst.extend_from_slice(&buf.inner.data[buf.inner.position..]);
    assert_eq!(*dst, b"llo world"[..]);
}

#[test]
fn test_get_mut_empty() {
    struct DummyBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl DummyBuf {
        fn new(data: Vec<u8>) -> Self {
            DummyBuf { data, position: 0 }
        }

        fn advance(&mut self, count: usize) {
            self.position += count;
        }
    }

    struct TakeBuf<T> {
        inner: T,
    }

    impl<T> TakeBuf<T> {
        fn new(inner: T) -> Self {
            TakeBuf { inner }
        }

        pub fn get_mut(&mut self) -> &mut T {
            &mut self.inner
        }
    }

    let mut buf = TakeBuf::new(DummyBuf::new(vec![]));
    let mut dst = vec![];

    // Simulating `put` functionality for example purposes
    dst.extend_from_slice(&buf.inner.data[buf.inner.position..]);
    assert_eq!(*dst, b""[..]);
}

