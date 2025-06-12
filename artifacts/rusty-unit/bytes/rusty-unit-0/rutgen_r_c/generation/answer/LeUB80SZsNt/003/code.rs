// Answer 0

#[test]
fn test_chunks_vectored_limit_zero() {
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

        fn advance(&mut self, _cnt: usize) {}

        fn copy_to_bytes(&mut self, _len: usize) -> crate::Bytes {
            unimplemented!()
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn chunks_vectored<'a>(&'a self, _dst: &mut [IoSlice<'a>]) -> usize {
            1
        }
    }

    let buf = TestBuf { data: vec![1, 2, 3] };
    let take = Take { inner: buf, limit: 0 };
    let mut slices: [IoSlice; 1] = [IoSlice::new(&mut [])];

    let result = take.chunks_vectored(&mut slices);
    assert_eq!(result, 0);
}

#[test]
#[should_panic]
fn test_chunks_vectored_panic_dst_len() {
    struct PanicBuf {
        data: Vec<u8>,
    }

    impl Buf for PanicBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.data
        }

        fn advance(&mut self, _cnt: usize) {}

        fn copy_to_bytes(&mut self, _len: usize) -> crate::Bytes {
            unimplemented!()
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
            if dst.is_empty() {
                panic!("dst is empty");
            }
            1
        }
    }

    let buf = PanicBuf { data: vec![1, 2, 3] };
    let take = Take { inner: buf, limit: 1 };
    let mut slices: [IoSlice; 0] = [];

    let _ = take.chunks_vectored(&mut slices);
}

#[test]
fn test_chunks_vectored_with_remaining() {
    struct TestBufWithChunks {
        data: Vec<u8>,
        index: usize,
    }

    impl Buf for TestBufWithChunks {
        fn remaining(&self) -> usize {
            self.data.len() - self.index
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.index..]
        }

        fn advance(&mut self, cnt: usize) {
            self.index += cnt;
        }

        fn copy_to_bytes(&mut self, _len: usize) -> crate::Bytes {
            unimplemented!()
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
            if dst.is_empty() {
                return 0;
            }
            1
        }
    }

    let buf = TestBufWithChunks { data: vec![1, 2, 3, 4, 5], index: 0 };
    let take = Take { inner: buf, limit: 5 };
    let mut slices: [IoSlice; 1] = [IoSlice::new(&mut [])];

    let result = take.chunks_vectored(&mut slices);
    assert_eq!(result, 1);
}

