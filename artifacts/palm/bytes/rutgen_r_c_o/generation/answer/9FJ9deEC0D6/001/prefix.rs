// Answer 0

#[test]
fn test_advancing_with_zero() {
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

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.remaining -= cnt;
        }
    }

    let mut buf = TestBufMut { remaining: 10 };
    let mut limit = Limit { inner: buf, limit: 10 };

    unsafe {
        limit.advance_mut(0);
    }
}

#[test]
fn test_advancing_with_equal_limit() {
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

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.remaining -= cnt;
        }
    }

    let mut buf = TestBufMut { remaining: 10 };
    let mut limit = Limit { inner: buf, limit: 10 };

    unsafe {
        limit.advance_mut(10);
    }
}

#[test]
#[should_panic]
fn test_advancing_exceeding_limit() {
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

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.remaining -= cnt;
        }
    }

    let mut buf = TestBufMut { remaining: 10 };
    let mut limit = Limit { inner: buf, limit: 10 };

    unsafe {
        limit.advance_mut(11);
    }
} 

#[test]
fn test_advancing_with_small_limit() {
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

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.remaining -= cnt;
        }
    }

    let mut buf = TestBufMut { remaining: 5 };
    let mut limit = Limit { inner: buf, limit: 5 };

    unsafe {
        limit.advance_mut(5);
    }
} 

#[test]
fn test_advancing_with_decreasing_limit() {
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

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.remaining -= cnt;
        }
    }

    let mut buf = TestBufMut { remaining: 15 };
    let mut limit = Limit { inner: buf, limit: 30 };

    unsafe {
        limit.advance_mut(15);
    }
} 

