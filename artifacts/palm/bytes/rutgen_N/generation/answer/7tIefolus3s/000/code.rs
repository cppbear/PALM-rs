// Answer 0

#[test]
fn test_put_with_contiguous_source() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new() -> Self {
            TestBuf {
                data: vec![1, 2, 3, 4, 5],
                position: 0,
            }
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn has_remaining(&self) -> bool {
            self.position < self.data.len()
        }

        fn chunk(&mut self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, n: usize) {
            self.position += n;
        }
    }

    let mut destination = Vec::new();
    let mut source = TestBuf::new();

    let remaining_before = source.remaining();
    destination.put(source);
    
    assert_eq!(destination.len(), remaining_before);
    assert_eq!(destination, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_put_with_empty_source() {
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

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn has_remaining(&self) -> bool {
            self.position < self.data.len()
        }

        fn chunk(&mut self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, n: usize) {
            self.position += n;
        }
    }

    let mut destination = Vec::new();
    let mut source = TestBuf::new();

    destination.put(source);
    
    assert!(destination.is_empty());
}

