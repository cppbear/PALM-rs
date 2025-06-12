// Answer 0

#[test]
fn test_advance_mut_with_enough_remaining_in_a() {
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
            self.remaining = self.remaining.saturating_sub(cnt);
        }
    }

    let mut buf_a = TestBufMut { remaining: 10 };
    let buf_b = TestBufMut { remaining: 5 };
    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(5);
    }

    assert_eq!(chain.a.remaining_mut(), 5);
    assert_eq!(chain.b.remaining_mut(), 5);
}

#[test]
fn test_advance_mut_with_partial_consumption_of_a() {
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
            self.remaining = self.remaining.saturating_sub(cnt);
        }
    }

    let mut buf_a = TestBufMut { remaining: 3 };
    let mut buf_b = TestBufMut { remaining: 5 };
    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(4);
    }

    assert_eq!(chain.a.remaining_mut(), 0);
    assert_eq!(chain.b.remaining_mut(), 1);
}

#[test]
fn test_advance_mut_with_no_remaining_in_a() {
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
            self.remaining = self.remaining.saturating_sub(cnt);
        }
    }

    let mut buf_a = TestBufMut { remaining: 0 };
    let mut buf_b = TestBufMut { remaining: 5 };
    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(3);
    }

    assert_eq!(chain.a.remaining_mut(), 0);
    assert_eq!(chain.b.remaining_mut(), 2);
}

#[test]
fn test_advance_mut_with_excessive_count() {
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
            self.remaining = self.remaining.saturating_sub(cnt);
        }
    }

    let mut buf_a = TestBufMut { remaining: 2 };
    let mut buf_b = TestBufMut { remaining: 1 };
    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(5);
    }

    assert_eq!(chain.a.remaining_mut(), 0);
    assert_eq!(chain.b.remaining_mut(), 0);
}

