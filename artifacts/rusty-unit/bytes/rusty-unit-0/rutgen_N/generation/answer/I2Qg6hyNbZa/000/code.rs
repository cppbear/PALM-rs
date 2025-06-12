// Answer 0

#[test]
fn test_advance_mut_with_valid_count() {
    struct BytesMut {
        cap: usize,
        len: usize,
    }

    impl BytesMut {
        fn len(&self) -> usize {
            self.len
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

    let mut buffer = BytesMut { cap: 10, len: 0 };
    unsafe {
        buffer.advance_mut(5);
    }
    assert_eq!(buffer.len(), 5);
}

#[test]
#[should_panic]
fn test_advance_mut_with_exceeding_count() {
    struct BytesMut {
        cap: usize,
        len: usize,
    }

    impl BytesMut {
        fn len(&self) -> usize {
            self.len
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

    let mut buffer = BytesMut { cap: 10, len: 10 };
    unsafe {
        buffer.advance_mut(1);
    }
}

