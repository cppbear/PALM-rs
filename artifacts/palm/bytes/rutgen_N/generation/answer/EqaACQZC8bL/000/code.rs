// Answer 0

#[test]
fn test_advance_mut_with_full_first_buf() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }
    }

    struct ChainBuf<'a> {
        a: &'a mut TestBuf,
        b: &'a mut TestBuf,
    }

    let mut buf_a = TestBuf::new(vec![1, 2, 3, 4, 5]);
    let mut buf_b = TestBuf::new(vec![6, 7, 8]);
    
    let mut chain = ChainBuf { a: &mut buf_a, b: &mut buf_b };

    unsafe { chain.advance_mut(3) };

    assert_eq!(chain.a.pos, 3);
    assert_eq!(chain.b.pos, 0);
}

#[test]
fn test_advance_mut_with_partial_first_buf() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }
    }

    struct ChainBuf<'a> {
        a: &'a mut TestBuf,
        b: &'a mut TestBuf,
    }

    let mut buf_a = TestBuf::new(vec![1, 2]);
    let mut buf_b = TestBuf::new(vec![3, 4, 5, 6]);

    let mut chain = ChainBuf { a: &mut buf_a, b: &mut buf_b };

    unsafe { chain.advance_mut(5) };

    assert_eq!(chain.a.pos, 2);
    assert_eq!(chain.b.pos, 3);
}

#[test]
fn test_advance_mut_with_exact_fit() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }
    }

    struct ChainBuf<'a> {
        a: &'a mut TestBuf,
        b: &'a mut TestBuf,
    }

    let mut buf_a = TestBuf::new(vec![1, 2, 3]);
    let mut buf_b = TestBuf::new(vec![4, 5]);

    let mut chain = ChainBuf { a: &mut buf_a, b: &mut buf_b };

    unsafe { chain.advance_mut(3) };
    
    assert_eq!(chain.a.pos, 3);
    assert_eq!(chain.b.pos, 0);
}

