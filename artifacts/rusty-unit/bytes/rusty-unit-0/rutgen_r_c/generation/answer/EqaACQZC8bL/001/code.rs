// Answer 0

#[test]
fn test_advance_mut_when_a_rem_equals_cnt() {
    struct BufMutImpl {
        data: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for BufMutImpl {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // This will be a stub, as this test doesn't use it.
            unimplemented!()
        }
    }

    let mut buf_a = BufMutImpl {
        data: vec![1, 2, 3, 4, 5],
        position: 0,
    };
    let mut buf_b = BufMutImpl {
        data: vec![6, 7, 8, 9, 10],
        position: 0,
    };
    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(5);
    }

    assert_eq!(chain.a.position, 5);
    assert_eq!(chain.b.position, 0);
}

#[test]
fn test_advance_mut_when_a_rem_greater_than_cnt() {
    struct BufMutImpl {
        data: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for BufMutImpl {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // This will be a stub, as this test doesn't use it.
            unimplemented!()
        }
    }

    let mut buf_a = BufMutImpl {
        data: vec![1, 2, 3, 4, 5],
        position: 0,
    };
    let mut buf_b = BufMutImpl {
        data: vec![6, 7, 8, 9, 10],
        position: 0,
    };
    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(3);
    }

    assert_eq!(chain.a.position, 3);
    assert_eq!(chain.b.position, 0);
}

#[test]
fn test_advance_mut_when_a_rem_is_zero() {
    struct BufMutImpl {
        data: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for BufMutImpl {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // This will be a stub, as this test doesn't use it.
            unimplemented!()
        }
    }

    let mut buf_a = BufMutImpl {
        data: vec![1, 2, 3, 4, 5],
        position: 5, // position points to the end
    };
    let mut buf_b = BufMutImpl {
        data: vec![6, 7, 8, 9, 10],
        position: 0,
    };
    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(5);
    }

    assert_eq!(chain.a.position, 5);
    assert_eq!(chain.b.position, 5); // All of b should be advanced
}

