// Answer 0

fn test_put_with_remaining() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }
    
    impl Buf for MockBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    let mut bytes_mut = BytesMut::new();
    let mut src = MockBuf {
        data: vec![1, 2, 3, 4, 5],
        position: 0,
    };

    bytes_mut.put(src);
    assert_eq!(bytes_mut.len(), 5);
    assert_eq!(bytes_mut.as_slice(), &[1, 2, 3, 4, 5]);
}

fn test_put_without_remaining() {
    struct EmptyBuf;

    impl Buf for EmptyBuf {
        fn remaining(&self) -> usize {
            0
        }

        fn chunk(&self) -> &[u8] {
            &[]
        }

        fn advance(&mut self, _: usize) {}
    }

    let mut bytes_mut = BytesMut::new();
    let src = EmptyBuf;

    bytes_mut.put(src);
    assert_eq!(bytes_mut.len(), 0);
    assert!(bytes_mut.is_empty());
}

