// Answer 0

#[test]
fn test_copy_to_bytes_a_rem_equals_len() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            let bytes = self.data[self.position..self.position + len].to_vec();
            self.position += len;
            Bytes::from(bytes)
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
    }

    let buf_a = TestBuf {
        data: vec![1, 2, 3, 4, 5],
        position: 0,
    };
    let buf_b = TestBuf {
        data: vec![6, 7, 8, 9, 10],
        position: 0,
    };
    let mut chain_buf = Chain { a: buf_a, b: buf_b };

    let result = chain_buf.copy_to_bytes(5);
    assert_eq!(result.len(), 5);
    assert_eq!(result.chunk(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_copy_to_bytes_a_rem_equals_len_zero_remaining_b() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            let bytes = self.data[self.position..self.position + len].to_vec();
            self.position += len;
            Bytes::from(bytes)
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
    }

    let buf_a = TestBuf {
        data: vec![], // Empty
        position: 0,
    };
    let buf_b = TestBuf {
        data: vec![6, 7, 8, 9, 10],
        position: 0,
    };
    let mut chain_buf = Chain { a: buf_a, b: buf_b };

    let result = chain_buf.copy_to_bytes(5);
    assert_eq!(result.len(), 5);
    assert_eq!(result.chunk(), &[6, 7, 8, 9, 10]);
}

