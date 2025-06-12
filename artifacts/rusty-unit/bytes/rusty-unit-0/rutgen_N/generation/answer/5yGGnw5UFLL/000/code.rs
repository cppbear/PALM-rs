// Answer 0

#[test]
fn test_set_limit() {
    struct TestBuf {
        data: Vec<u8>,
        limit: usize,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data, limit: data.len(), position: 0 }
        }

        fn take(&mut self, limit: usize) -> &mut Self {
            self.limit = limit;
            self
        }

        fn set_limit(&mut self, lim: usize) {
            self.limit = lim;
        }

        fn put(&mut self, dst: &mut Vec<u8>) {
            let bytes_to_read = self.limit.min(self.data.len() - self.position);
            dst.extend_from_slice(&self.data[self.position..self.position + bytes_to_read]);
            self.position += bytes_to_read;
        }
    }

    let mut buf = TestBuf::new(b"hello world".to_vec()).take(2);
    let mut dst = vec![];

    buf.put(&mut dst);
    assert_eq!(*dst, b"he"[..]);

    dst.clear();

    buf.set_limit(3);
    buf.put(&mut dst);
    assert_eq!(*dst, b"llo"[..]);
}

#[test]
fn test_set_limit_exceeding() {
    struct TestBuf {
        data: Vec<u8>,
        limit: usize,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data, limit: data.len(), position: 0 }
        }

        fn take(&mut self, limit: usize) -> &mut Self {
            self.limit = limit;
            self
        }

        fn set_limit(&mut self, lim: usize) {
            self.limit = lim;
        }

        fn put(&mut self, dst: &mut Vec<u8>) {
            let bytes_to_read = self.limit.min(self.data.len() - self.position);
            dst.extend_from_slice(&self.data[self.position..self.position + bytes_to_read]);
            self.position += bytes_to_read;
        }
    }

    let mut buf = TestBuf::new(b"hello world".to_vec()).take(100);
    let mut dst = vec![];

    buf.put(&mut dst);
    assert_eq!(*dst, b"hello world"[..]);

    dst.clear();

    buf.set_limit(0);
    buf.put(&mut dst);
    assert_eq!(dst.len(), 0);
}

#[test]
fn test_set_limit_empty_buf() {
    struct TestBuf {
        data: Vec<u8>,
        limit: usize,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data, limit: data.len(), position: 0 }
        }

        fn take(&mut self, limit: usize) -> &mut Self {
            self.limit = limit;
            self
        }

        fn set_limit(&mut self, lim: usize) {
            self.limit = lim;
        }

        fn put(&mut self, dst: &mut Vec<u8>) {
            let bytes_to_read = self.limit.min(self.data.len() - self.position);
            dst.extend_from_slice(&self.data[self.position..self.position + bytes_to_read]);
            self.position += bytes_to_read;
        }
    }

    let mut buf = TestBuf::new(Vec::new()).take(10);
    let mut dst = vec![];

    buf.set_limit(5);
    buf.put(&mut dst);
    assert_eq!(dst.len(), 0);
}

