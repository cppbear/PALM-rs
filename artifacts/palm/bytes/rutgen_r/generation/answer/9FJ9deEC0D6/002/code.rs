// Answer 0

#[test]
#[should_panic] // This test expects a panic due to the assertion failure.
fn test_advance_mut_panic_due_to_exceeding_limit() {
    struct Inner {
        data: Vec<u8>,
        position: usize,
    }

    impl Inner {
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    struct Buf {
        inner: Inner,
        limit: usize,
    }

    let mut inner = Inner {
        data: vec![0; 10],
        position: 0,
    };
    
    let mut buf = Buf {
        inner,
        limit: 5,
    };

    // Attempt to advance beyond the limit
    unsafe {
        buf.advance_mut(6);
    }
}

