// Answer 0

#[test]
fn test_chunk_with_remaining_a() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
        
        fn copy_to_slice(&mut self, dst: &mut [u8]) {
            let rem = self.remaining();
            let to_copy = rem.min(dst.len());
            dst[..to_copy].copy_from_slice(&self.data[self.pos..self.pos + to_copy]);
            self.advance(to_copy);
        }
    }

    let buf_a = TestBuf {
        data: vec![1, 2, 3, 4, 5], // a has remaining data
        pos: 0,
    };
    let buf_b = TestBuf {
        data: vec![6, 7, 8, 9, 10],
        pos: 0,
    };

    let chain = Chain { a: buf_a, b: buf_b };
    let result = chain.chunk();
}

#[test]
fn test_chunk_with_empty_a_and_remaining_b() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.pos..]
        }

        fn advance(&mut self, cnt: usize) {
            self.pos += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
        
        fn copy_to_slice(&mut self, dst: &mut [u8]) {
            let rem = self.remaining();
            let to_copy = rem.min(dst.len());
            dst[..to_copy].copy_from_slice(&self.data[self.pos..self.pos + to_copy]);
            self.advance(to_copy);
        }
    }

    let buf_a = TestBuf {
        data: vec![], // a has no remaining data
        pos: 0,
    };
    let buf_b = TestBuf {
        data: vec![11, 12, 13, 14, 15], // b has remaining data
        pos: 0,
    };

    let chain = Chain { a: buf_a, b: buf_b };
    let result = chain.chunk();
}

