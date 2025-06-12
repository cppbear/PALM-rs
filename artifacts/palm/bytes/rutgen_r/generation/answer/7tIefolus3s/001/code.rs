// Answer 0

#[test]
fn test_put_with_remaining_data() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new() -> Self {
            TestBuf {
                data: Vec::new(),
                position: 0,
            }
        }

        fn reserve(&mut self, additional: usize) {
            self.data.reserve(additional);
        }

        fn extend_from_slice(&mut self, chunk: &[u8]) {
            self.data.extend_from_slice(chunk);
        }
        
        fn has_remaining(&self) -> bool {
            self.position < self.data.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, len: usize) {
            self.position += len;
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
    }

    let mut buf = TestBuf::new();
    buf.data = vec![1, 2, 3, 4, 5]; // mock data
    let mut src = TestBuf::new();
    src.data = vec![6, 7, 8]; // source data with remaining

    buf.put(src);

    assert_eq!(buf.data, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
#[should_panic]
fn test_put_with_no_remaining_data() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new() -> Self {
            TestBuf {
                data: Vec::new(),
                position: 0,
            }
        }

        fn reserve(&mut self, additional: usize) {
            self.data.reserve(additional);
        }

        fn extend_from_slice(&mut self, chunk: &[u8]) {
            self.data.extend_from_slice(chunk);
        }

        fn has_remaining(&self) -> bool {
            self.position < self.data.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, len: usize) {
            self.position += len;
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
    }

    let mut buf = TestBuf::new();
    buf.data = vec![1, 2, 3, 4, 5]; // mock data
    let mut src = TestBuf::new(); // src is empty

    buf.put(src); // Should panic because src has no remaining data
}

