// Answer 0

#[test]
fn test_put_with_empty_src() {
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
    let src = MockBuf { data: vec![], position: 0 };

    // The src has no remaining data
    bytes_mut.put(src);

    assert_eq!(bytes_mut.len(), 0);
}

#[test]
fn test_put_with_src_having_no_remaining() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for MockBuf {
        fn remaining(&self) -> usize {
            0
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..self.position]
        }

        fn advance(&mut self, _: usize) {
            // Does nothing since there is no remaining data
        }
    }

    let mut bytes_mut = BytesMut::new();
    let src = MockBuf { data: vec![1, 2, 3], position: 0 };

    // The src has no remaining data due to the overridden remaining method
    bytes_mut.put(src);

    assert_eq!(bytes_mut.len(), 0);
}

#[test]
fn test_put_with_src_empty_chunk() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for MockBuf {
        fn remaining(&self) -> usize {
            1  // Only one byte remaining
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..self.position + 0]  // Empty chunk
        }

        fn advance(&mut self, _: usize) {
            // Advance does nothing as the size is zero
        }
    }

    let mut bytes_mut = BytesMut::new();
    let src = MockBuf { data: vec![1, 2, 3], position: 0 };

    // The src returns an empty chunk to put
    bytes_mut.put(src);

    assert_eq!(bytes_mut.len(), 0);
}

