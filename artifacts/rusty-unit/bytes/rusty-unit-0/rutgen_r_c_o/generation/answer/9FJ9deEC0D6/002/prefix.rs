// Answer 0

#[test]
#[should_panic]
fn test_advance_mut_exceeding_limit() {
    struct TestBufMut {
        limit: usize,
        counter: usize,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.limit
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.counter += cnt;
        }
    }

    let limit = 5;
    let mut test_buf = TestBufMut { limit, counter: 0 };
    let excess_count = limit + 1; // cnt > self.limit

    unsafe {
        test_buf.advance_mut(excess_count);
    }
}

#[test]
#[should_panic]
fn test_advance_mut_max_value() {
    struct TestBufMut {
        limit: usize,
        counter: usize,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.limit
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.counter += cnt;
        }
    }

    let limit = 10;
    let mut test_buf = TestBufMut { limit, counter: 0 };
    let max_count = usize::MAX; // cnt is max value

    unsafe {
        test_buf.advance_mut(max_count);
    }
}

