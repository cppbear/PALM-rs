// Answer 0

#[test]
fn test_chain_chunk_a_remaining() {
    struct MockBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for MockBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }
    }

    let buf_a = MockBuf { data: vec![1, 2, 3], pos: 0 };
    let buf_b = MockBuf { data: vec![4, 5, 6], pos: 0 };
    let chain = Chain { a: buf_a, b: buf_b };

    assert_eq!(chain.chunk(), &[1, 2, 3]);
}

#[test]
fn test_chain_chunk_b_remaining() {
    struct MockBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for MockBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }
    }

    let buf_a = MockBuf { data: vec![1, 2, 3], pos: 3 };
    let buf_b = MockBuf { data: vec![4, 5, 6], pos: 0 };
    let chain = Chain { a: buf_a, b: buf_b };

    assert_eq!(chain.chunk(), &[4, 5, 6]);
}

#[test]
fn test_chain_chunk_no_remaining() {
    struct MockBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for MockBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }
    }

    let buf_a = MockBuf { data: vec![1, 2, 3], pos: 3 };
    let buf_b = MockBuf { data: vec![], pos: 0 };
    let chain = Chain { a: buf_a, b: buf_b };

    assert_eq!(chain.chunk(), &[]);
}

