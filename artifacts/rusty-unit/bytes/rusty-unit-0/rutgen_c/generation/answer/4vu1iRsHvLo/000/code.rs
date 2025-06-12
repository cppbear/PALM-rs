// Answer 0

#[test]
fn test_chunks_vectored_with_empty_dst() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.data
        }

        fn advance(&mut self, cnt: usize) {
            self.data.drain(0..cnt);
        }

        fn has_remaining(&self) -> bool {
            !self.data.is_empty()
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            unimplemented!()
        }
    }

    let buf_a = TestBuf { data: vec![1, 2, 3] };
    let buf_b = TestBuf { data: vec![4, 5] };
    let chain_buf = Chain { a: buf_a, b: buf_b };

    let mut dst: [IoSlice; 0] = [];
    assert_eq!(chain_buf.chunks_vectored(&mut dst), 0);
}

#[test]
fn test_chunks_vectored_with_remaining_data() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.data
        }

        fn advance(&mut self, cnt: usize) {
            self.data.drain(0..cnt);
        }

        fn has_remaining(&self) -> bool {
            !self.data.is_empty()
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            unimplemented!()
        }
    }

    let buf_a = TestBuf { data: vec![1, 2] };
    let buf_b = TestBuf { data: vec![3, 4] };
    let chain_buf = Chain { a: buf_a, b: buf_b };

    let mut dst = [IoSlice::new(&mut [0; 4])];
    assert_eq!(chain_buf.chunks_vectored(&mut dst), 1);
    assert_eq!(dst[0].len(), 2);
}

#[test]
fn test_chunks_vectored_combined_data() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.data
        }

        fn advance(&mut self, cnt: usize) {
            self.data.drain(0..cnt);
        }

        fn has_remaining(&self) -> bool {
            !self.data.is_empty()
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            unimplemented!()
        }
    }

    let buf_a = TestBuf { data: vec![1, 2, 3] };
    let buf_b = TestBuf { data: vec![4, 5] };
    let chain_buf = Chain { a: buf_a, b: buf_b };

    let mut dst = [IoSlice::new(&mut [0; 10])];
    assert_eq!(chain_buf.chunks_vectored(&mut dst), 1);
    assert_eq!(dst[0].len(), 5);
    assert_eq!(&dst[0][..5], &[1, 2, 3, 4, 5]);
}

