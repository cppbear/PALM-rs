// Answer 0

#[test]
fn test_chunk() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl AsRef<[u8]> for TestBuf {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }

    impl TestBuf {
        fn position(&self) -> usize {
            self.position
        }
        
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    let buf = TestBuf {
        data: vec![1, 2, 3, 4, 5],
        position: 2,
    };

    let result = buf.chunk();
    assert_eq!(result, &[3, 4, 5]);
}

#[test]
fn test_chunk_exceeding_position() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl AsRef<[u8]> for TestBuf {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }

    impl TestBuf {
        fn position(&self) -> usize {
            self.position
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
        
        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    let buf = TestBuf {
        data: vec![1, 2, 3, 4, 5],
        position: 10,
    };

    let result = buf.chunk();
    assert!(result.is_empty());
}

