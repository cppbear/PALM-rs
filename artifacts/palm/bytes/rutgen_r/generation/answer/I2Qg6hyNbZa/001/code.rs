// Answer 0

#[test]
#[should_panic]
fn test_advance_mut_panics_when_cnt_exceeds_remaining() {
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

        fn cap(&self) -> usize {
            self.cap
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            let remaining = self.cap - self.len();
            if cnt > remaining {
                super::panic_advance(&TryGetError {
                    requested: cnt,
                    available: remaining,
                });
            }
            self.len = self.len() + cnt;
        }
    }

    struct TryGetError {
        requested: usize,
        available: usize,
    }

    let mut bytes = BytesMut::new(10);
    let cnt = 15; // cnt > remaining (remaining = 10 - 0 = 10)

    unsafe {
        bytes.advance_mut(cnt);
    }
}

#[test]
fn test_advance_mut_does_not_panic_when_cnt_is_within_bounds() {
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

        fn cap(&self) -> usize {
            self.cap
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            let remaining = self.cap - self.len();
            if cnt > remaining {
                super::panic_advance(&TryGetError {
                    requested: cnt,
                    available: remaining,
                });
            }
            self.len = self.len() + cnt;
        }
    }

    struct TryGetError {
        requested: usize,
        available: usize,
    }

    let mut bytes = BytesMut::new(10);
    let cnt = 5; // cnt <= remaining (remaining = 10 - 0 = 10)

    unsafe {
        bytes.advance_mut(cnt); // Should not panic
        assert_eq!(bytes.len(), 5); // Ensure the length is updated correctly
    }
}

