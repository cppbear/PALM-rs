// Answer 0

#[test]
fn test_chunk_mut_for_uninit_slice() {
    struct TestBufMut {
        data: [core::mem::MaybeUninit<u8>; 10],
        offset: usize,
    }

    impl TestBufMut {
        fn new() -> Self {
            Self {
                data: Default::default(),
                offset: 0,
            }
        }
    }

    unsafe impl BufMut for TestBufMut {
        #[inline]
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.offset
        }

        #[inline]
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            UninitSlice(&mut self.data[self.offset..])
        }

        #[inline]
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.offset += cnt;
        }

        #[inline]
        fn put_slice(&mut self, _src: &[u8]) {
            // Simulating copy for the sake of the example
        }

        #[inline]
        fn put_bytes(&mut self, _val: u8, _cnt: usize) {
            // Simulating write for the sake of the example
        }
    }

    let mut buf = TestBufMut::new();
    let chunk = buf.chunk_mut();
    assert_eq!(chunk.0.len(), 10); // Full capacity of the buffer

    unsafe {
        buf.advance_mut(5);
    }

    let chunk_after_advance = buf.chunk_mut();
    assert_eq!(chunk_after_advance.0.len(), 5); // Remaining length after advance
}

#[test]
#[should_panic]
fn test_chunk_mut_panic_conditions() {
    struct PanicTestBufMut {
        data: [core::mem::MaybeUninit<u8>; 5],
        offset: usize,
    }

    impl PanicTestBufMut {
        fn new() -> Self {
            Self {
                data: Default::default(),
                offset: 5, // Offset equals the length to provoke a panic
            }
        }
    }

    unsafe impl BufMut for PanicTestBufMut {
        #[inline]
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.offset
        }

        #[inline]
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            if self.remaining_mut() == 0 {
                panic!("No remaining mutable buffer");
            }
            UninitSlice(&mut self.data[self.offset..])
        }

        #[inline]
        unsafe fn advance_mut(&mut self, _cnt: usize) {}

        #[inline]
        fn put_slice(&mut self, _src: &[u8]) {}

        #[inline]
        fn put_bytes(&mut self, _val: u8, _cnt: usize) {}
    }

    let mut panic_buf = PanicTestBufMut::new();
    panic_buf.chunk_mut(); // This should panic
}

