// Answer 0

#[test]
fn test_set_limit_with_valid_value() {
    struct TestBuf {
        limit: usize,
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>, limit: usize) -> Self {
            Self { limit, data, position: 0 }
        }

        fn take(&mut self, _lim: usize) -> &mut Self {
            self
        }

        fn put(&mut self, dst: &mut Vec<u8>) {
            let bytes_to_copy = self.data[self.position..].len().min(self.limit);
            dst.extend_from_slice(&self.data[self.position..self.position + bytes_to_copy]);
            self.position += bytes_to_copy;
        }

        fn set_limit(&mut self, lim: usize) {
            self.limit = lim;
        }
    }

    let mut buf = TestBuf::new(b"hello world".to_vec(), 2);
    let mut dst = vec![];

    buf.take(2);
    buf.put(&mut dst);
    assert_eq!(*dst, b"he"[..]);

    dst.clear();

    buf.set_limit(3);
    buf.put(&mut dst);
    assert_eq!(*dst, b"llo"[..]);
}

#[test]
fn test_set_limit_to_zero() {
    struct TestBuf {
        limit: usize,
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>, limit: usize) -> Self {
            Self { limit, data, position: 0 }
        }

        fn take(&mut self, _lim: usize) -> &mut Self {
            self
        }

        fn put(&mut self, dst: &mut Vec<u8>) {
            let bytes_to_copy = self.data[self.position..].len().min(self.limit);
            dst.extend_from_slice(&self.data[self.position..self.position + bytes_to_copy]);
            self.position += bytes_to_copy;
        }

        fn set_limit(&mut self, lim: usize) {
            self.limit = lim;
        }
    }

    let mut buf = TestBuf::new(b"hello world".to_vec(), 2);
    let mut dst = vec![];

    buf.set_limit(0);
    buf.put(&mut dst);
    assert_eq!(dst.len(), 0);
}

#[test]
fn test_set_limit_exceeding_data_length() {
    struct TestBuf {
        limit: usize,
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>, limit: usize) -> Self {
            Self { limit, data, position: 0 }
        }

        fn take(&mut self, _lim: usize) -> &mut Self {
            self
        }

        fn put(&mut self, dst: &mut Vec<u8>) {
            let bytes_to_copy = self.data[self.position..].len().min(self.limit);
            dst.extend_from_slice(&self.data[self.position..self.position + bytes_to_copy]);
            self.position += bytes_to_copy;
        }

        fn set_limit(&mut self, lim: usize) {
            self.limit = lim;
        }
    }

    let mut buf = TestBuf::new(b"hello world".to_vec(), 2);
    let mut dst = vec![];

    buf.set_limit(12); // Exceeding the length of "hello world"
    buf.put(&mut dst);
    assert_eq!(*dst, b"hello world"[..]);
}

