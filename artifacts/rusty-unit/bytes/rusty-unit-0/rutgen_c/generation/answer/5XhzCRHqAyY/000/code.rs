// Answer 0

#[test]
fn test_remaining_mut_within_limit() {
    struct TestBuf {
        remaining: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, _: usize) {
            unimplemented!()
        }
    }

    let test_buf = TestBuf { remaining: 8 };
    let limit_v = Limit { inner: test_buf, limit: 10 };
    assert_eq!(limit_v.remaining_mut(), 8);
}

#[test]
fn test_remaining_mut_at_limit() {
    struct TestBuf {
        remaining: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, _: usize) {
            unimplemented!()
        }
    }

    let test_buf = TestBuf { remaining: 10 };
    let limit_v = Limit { inner: test_buf, limit: 10 };
    assert_eq!(limit_v.remaining_mut(), 10);
}

#[test]
fn test_remaining_mut_exceeds_limit() {
    struct TestBuf {
        remaining: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        unsafe fn advance_mut(&mut self, _: usize) {
            unimplemented!()
        }
    }

    let test_buf = TestBuf { remaining: 15 };
    let limit_v = Limit { inner: test_buf, limit: 10 };
    assert_eq!(limit_v.remaining_mut(), 10);
}

