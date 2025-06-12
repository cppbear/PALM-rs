// Answer 0

#[test]
fn test_advance_mut_with_max_limit() {
    struct TestBufMut {
        data: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
    }

    let limit_value: usize = 10;
    let mut buf = TestBufMut {
        data: vec![0; limit_value],
        position: 0,
    };
    let mut limit_wrapper = Limit {
        inner: buf,
        limit: limit_value,
    };

    unsafe {
        limit_wrapper.advance_mut(limit_value); // should not panic
    }

    assert_eq!(limit_wrapper.inner.remaining_mut(), 0);
}

#[test]
#[should_panic]
fn test_advance_mut_with_exceeding_limit() {
    struct TestBufMut {
        data: Vec<u8>,
        position: usize,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
    }

    let limit_value: usize = 10;
    let mut buf = TestBufMut {
        data: vec![0; limit_value],
        position: 0,
    };
    let mut limit_wrapper = Limit {
        inner: buf,
        limit: limit_value,
    };

    unsafe {
        limit_wrapper.advance_mut(limit_value + 1); // should panic
    }
}

