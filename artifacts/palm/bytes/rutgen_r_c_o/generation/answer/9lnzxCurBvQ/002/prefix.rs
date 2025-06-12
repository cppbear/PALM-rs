// Answer 0

#[test]
fn test_chunk_mut_a_has_no_remaining() {
    struct MockBufMutA {
        remaining: usize,
    }

    impl BufMut for MockBufMutA {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice([MaybeUninit::new(0)])
        }
    }

    struct MockBufMutB {
        remaining: usize,
    }

    impl BufMut for MockBufMutB {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice([MaybeUninit::new(1)])
        }
    }

    let mut buf_a = MockBufMutA { remaining: 0 };
    let mut buf_b = MockBufMutB { remaining: 16 }; // Some non-zero remaining

    let mut chain = Chain { a: buf_a, b: buf_b };

    let chunk = chain.chunk_mut();
}

#[test]
fn test_chunk_mut_a_has_no_remaining_b_has_full() {
    struct MockBufMutA {
        remaining: usize,
    }

    impl BufMut for MockBufMutA {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice([MaybeUninit::new(0)])
        }
    }

    struct MockBufMutB {
        remaining: usize,
    }

    impl BufMut for MockBufMutB {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice([MaybeUninit::new(1); 1024]) // 1024 bytes
        }
    }

    let mut buf_a = MockBufMutA { remaining: 0 };
    let mut buf_b = MockBufMutB { remaining: 1024 }; // Full capacity

    let mut chain = Chain { a: buf_a, b: buf_b };

    let chunk = chain.chunk_mut();
}

#[test]
fn test_chunk_mut_a_has_no_remaining_b_has_single() {
    struct MockBufMutA {
        remaining: usize,
    }

    impl BufMut for MockBufMutA {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice([MaybeUninit::new(0)])
        }
    }

    struct MockBufMutB {
        remaining: usize,
    }

    impl BufMut for MockBufMutB {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut UninitSlice([MaybeUninit::new(1)])
        }
    }

    let mut buf_a = MockBufMutA { remaining: 0 };
    let mut buf_b = MockBufMutB { remaining: 1 };

    let mut chain = Chain { a: buf_a, b: buf_b };

    let chunk = chain.chunk_mut();
}

