// Answer 0

#[test]
fn test_remaining_mut_with_non_empty_buffers() {
    struct TestBufMut {
        remaining: usize,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, _cnt: usize) {
            unimplemented!()
        }
    }

    let buf_a = TestBufMut { remaining: 10 };
    let buf_b = TestBufMut { remaining: 5 };
    let chain = Chain { a: buf_a, b: buf_b };

    assert_eq!(chain.remaining_mut(), 15);
}

#[test]
fn test_remaining_mut_with_empty_buffers() {
    struct TestBufMut {
        remaining: usize,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, _cnt: usize) {
            unimplemented!()
        }
    }

    let buf_a = TestBufMut { remaining: 0 };
    let buf_b = TestBufMut { remaining: 0 };
    let chain = Chain { a: buf_a, b: buf_b };

    assert_eq!(chain.remaining_mut(), 0);
}

#[test]
fn test_remaining_mut_with_one_empty_buffer() {
    struct TestBufMut {
        remaining: usize,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, _cnt: usize) {
            unimplemented!()
        }
    }

    let buf_a = TestBufMut { remaining: 10 };
    let buf_b = TestBufMut { remaining: 0 };
    let chain = Chain { a: buf_a, b: buf_b };

    assert_eq!(chain.remaining_mut(), 10);
}

#[test]
fn test_remaining_mut_with_different_sizes() {
    struct TestBufMut {
        remaining: usize,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, _cnt: usize) {
            unimplemented!()
        }
    }

    let buf_a = TestBufMut { remaining: 20 };
    let buf_b = TestBufMut { remaining: 30 };
    let chain = Chain { a: buf_a, b: buf_b };

    assert_eq!(chain.remaining_mut(), 50);
}

