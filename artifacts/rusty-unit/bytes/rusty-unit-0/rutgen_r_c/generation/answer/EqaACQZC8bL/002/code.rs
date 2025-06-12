// Answer 0

#[test]
fn test_advance_mut_with_remaining_a_not_zero_and_cnt_exceeds_a_rem() {
    struct MockBufMut {
        remaining: usize,
    }

    unsafe impl BufMut for MockBufMut {
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

    let mut buf_a = MockBufMut { remaining: 10 };
    let mut buf_b = MockBufMut { remaining: 0 };

    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(15);
    }

    // Check the state after advance_mut
    assert_eq!(chain.a.remaining_mut(), 0); // a should be consumed completely
    assert_eq!(chain.b.remaining_mut(), 0); // b should still be unmodified
}

#[test]
fn test_advance_mut_with_a_fully_consumed_and_cnt_greater_than_zero() {
    struct MockBufMut {
        remaining: usize,
    }

    unsafe impl BufMut for MockBufMut {
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

    let mut buf_a = MockBufMut { remaining: 0 }; // A consumed
    let mut buf_b = MockBufMut { remaining: 20 }; // B has remaining bytes

    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(5);
    }

    // Check the state after advance_mut
    assert_eq!(chain.a.remaining_mut(), 0); // a remains consumed
    assert_eq!(chain.b.remaining_mut(), 15); // b should have decreased
}

#[test]
fn test_advance_mut_with_a_rem_not_zero_and_cnt_equal_to_a_rem() {
    struct MockBufMut {
        remaining: usize,
    }

    unsafe impl BufMut for MockBufMut {
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

    let mut buf_a = MockBufMut { remaining: 15 };
    let mut buf_b = MockBufMut { remaining: 5 };

    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(15);
    }

    // Check state after advance_mut
    assert_eq!(chain.a.remaining_mut(), 0); // a should be fully advanced
    assert_eq!(chain.b.remaining_mut(), 5); // b remains unchanged
}

