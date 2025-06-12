// Answer 0

#[test]
fn test_chunk_mut_a_has_remaining() {
    struct TestBufMut {
        remaining: usize,
        chunk: UninitSlice,
    }

    impl TestBufMut {
        fn new(remaining: usize) -> Self {
            let chunk = UninitSlice([std::mem::MaybeUninit::new(0); 10]);
            Self { remaining, chunk }
        }
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut self.chunk
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.remaining = self.remaining.saturating_sub(cnt);
        }
    }

    let mut buf_a = TestBufMut::new(5);
    let buf_b = TestBufMut::new(0);

    let mut chain = Chain {
        a: buf_a,
        b: buf_b,
    };

    let chunk = chain.chunk_mut();
    assert_eq!(chunk.0.len(), 10);
}

#[test]
fn test_chunk_mut_b_when_a_has_no_remaining() {
    struct TestBufMut {
        remaining: usize,
        chunk: UninitSlice,
    }

    impl TestBufMut {
        fn new(remaining: usize) -> Self {
            let chunk = UninitSlice([std::mem::MaybeUninit::new(0); 10]);
            Self { remaining, chunk }
        }
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut self.chunk
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.remaining = self.remaining.saturating_sub(cnt);
        }
    }

    let buf_a = TestBufMut::new(0);
    let mut buf_b = TestBufMut::new(5);

    let mut chain = Chain {
        a: buf_a,
        b: buf_b,
    };

    let chunk = chain.chunk_mut();
    assert_eq!(chunk.0.len(), 10);
}

