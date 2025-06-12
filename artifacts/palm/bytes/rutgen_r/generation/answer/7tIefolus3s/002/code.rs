// Answer 0

#[test]
fn test_put_with_no_remaining() {
    struct EmptyBuf;

    impl super::Buf for EmptyBuf {
        fn remaining(&self) -> usize {
            0
        }

        fn has_remaining(&self) -> bool {
            false
        }

        fn chunk(&self) -> &[u8] {
            &[]
        }

        fn advance(&mut self, _: usize) {}
    }

    let mut destination = Vec::new();
    let mut source = EmptyBuf;

    destination.put(source);

    assert_eq!(destination.len(), 0);
}

#[test]
fn test_put_with_non_empty_source() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl super::Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn has_remaining(&self) -> bool {
            self.position < self.data.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, l: usize) {
            self.position += l;
        }
    }

    let mut destination = Vec::new();
    let mut source = TestBuf {
        data: vec![1, 2, 3],
        position: 0,
    };

    destination.put(source);

    assert_eq!(destination.len(), 3);
    assert_eq!(destination, vec![1, 2, 3]);
}

