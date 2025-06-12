// Answer 0

#[test]
fn test_advance_mut_boundary_condition() {
    struct BytesMut {
        cap: usize,
        len: usize,
    }

    impl BytesMut {
        fn new(cap: usize) -> Self {
            BytesMut { cap, len: 0 }
        }
        
        fn len(&self) -> usize {
            self.len
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            let remaining = self.cap - self.len();
            if cnt > remaining {
                panic("cnt is greater than remaining");
            }
            self.len = self.len() + cnt;
        }
    }

    let mut buf = BytesMut::new(10);
    let cnt = 10; // This should match the remaining capacity

    // Valid operation that does not panic
    unsafe {
        buf.advance_mut(cnt);
    }

    assert_eq!(buf.len(), 10);
}

#[should_panic(expected = "cnt is greater than remaining")]
#[test]
fn test_advance_mut_panic_condition() {
    struct BytesMut {
        cap: usize,
        len: usize,
    }

    impl BytesMut {
        fn new(cap: usize) -> Self {
            BytesMut { cap, len: 0 }
        }
        
        fn len(&self) -> usize {
            self.len
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            let remaining = self.cap - self.len();
            if cnt > remaining {
                panic("cnt is greater than remaining");
            }
            self.len = self.len() + cnt;
        }
    }

    let mut buf = BytesMut::new(10);
    let cnt = 11; // This should exceed the remaining capacity

    // This should panic
    unsafe {
        buf.advance_mut(cnt);
    }
}

