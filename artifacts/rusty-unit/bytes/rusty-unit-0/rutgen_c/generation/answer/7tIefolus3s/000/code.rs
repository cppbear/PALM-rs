// Answer 0

#[test]
fn test_put_with_empty_src() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl super::Buf for MockBuf {
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

    let mut buf = Vec::new();
    let src = MockBuf { data: Vec::new(), position: 0 };
    buf.put(src);
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_put_with_contiguous_src() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl super::Buf for MockBuf {
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

    let mut buf = Vec::new();
    let src = MockBuf { data: vec![1, 2, 3, 4], position: 0 };
    buf.put(src);
    assert_eq!(buf, vec![1, 2, 3, 4]);
}

#[test]
fn test_put_with_partial_advance() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl super::Buf for MockBuf {
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

    let mut buf = Vec::new();
    let src = MockBuf { data: vec![1, 2, 3], position: 0 };
    buf.put(src);
    assert_eq!(buf, vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_put_with_insufficient_capacity() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl super::Buf for MockBuf {
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

    let mut buf = Vec::with_capacity(2);
    let src = MockBuf { data: vec![1, 2, 3], position: 0 };
    buf.put(src); // This should panic due to insufficient capacity
}

