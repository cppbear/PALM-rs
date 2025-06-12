// Answer 0

#[test]
fn test_chunk_mut_with_remaining_a() {
    struct TestBufMutA {
        remaining: usize,
        slice: UninitSlice,
    }
    
    impl TestBufMutA {
        fn new(remaining: usize) -> Self {
            let slice = UninitSlice([MaybeUninit::uninit(); 1024]);
            Self { remaining, slice }
        }
    }
    
    unsafe impl BufMut for TestBufMutA {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut self.slice
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.remaining -= cnt;
        }
    }

    struct TestBufMutB {
        remaining: usize,
        slice: UninitSlice,
    }

    unsafe impl BufMut for TestBufMutB {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut self.slice
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.remaining -= cnt;
        }
    }

    let a = TestBufMutA::new(50);
    let b = TestBufMutB { remaining: 0, slice: UninitSlice([MaybeUninit::uninit(); 1024]) };

    let mut chain = Chain { a, b };
    let result = chain.chunk_mut();
}

#[test]
fn test_chunk_mut_with_remaining_b() {
    struct TestBufMutA {
        remaining: usize,
        slice: UninitSlice,
    }
    
    impl TestBufMutA {
        fn new(remaining: usize) -> Self {
            let slice = UninitSlice([MaybeUninit::uninit(); 1024]);
            Self { remaining, slice }
        }
    }

    unsafe impl BufMut for TestBufMutA {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut self.slice
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.remaining -= cnt;
        }
    }

    struct TestBufMutB {
        remaining: usize,
        slice: UninitSlice,
    }

    unsafe impl BufMut for TestBufMutB {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            &mut self.slice
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.remaining -= cnt;
        }
    }

    let a = TestBufMutA::new(0);
    let b = TestBufMutB { remaining: 30, slice: UninitSlice([MaybeUninit::uninit(); 1024]) };

    let mut chain = Chain { a, b };
    let result = chain.chunk_mut();
}

