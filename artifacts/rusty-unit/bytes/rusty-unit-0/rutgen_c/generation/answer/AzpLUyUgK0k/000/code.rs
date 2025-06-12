// Answer 0

#[test]
fn test_chunk_mut() {
    struct TestBufMut {
        data: [core::mem::MaybeUninit<u8>; 8],
        cursor: usize,
    }

    impl TestBufMut {
        fn new() -> Self {
            Self {
                data: Default::default(),
                cursor: 0,
            }
        }
    }

    unsafe impl BufMut for TestBufMut {
        #[inline]
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.cursor
        }

        #[inline]
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let len = self.remaining_mut();
            let slice = &mut self.data[self.cursor..self.cursor + len];
            unsafe { UninitSlice(slice) }
        }

        #[inline]
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.cursor += cnt;
        }

        #[inline]
        fn put_slice(&mut self, _src: &[u8]) {
            // Stub implementation for testing
        }

        #[inline]
        fn put_bytes(&mut self, _val: u8, _cnt: usize) {
            // Stub implementation for testing
        }
    }

    let mut buf = TestBufMut::new();
    let chunk = buf.chunk_mut();
    assert_eq!(buf.remaining_mut(), 8);
}

