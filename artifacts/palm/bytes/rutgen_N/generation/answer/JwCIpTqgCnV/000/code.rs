// Answer 0

#[test]
fn test_advance_mut_within_capacity() {
    struct BufMut {
        data: Vec<u8>,
        len: usize,
        capacity: usize,
    }

    impl BufMut {
        fn new(capacity: usize) -> Self {
            let data = Vec::with_capacity(capacity);
            Self { data, len: 0, capacity }
        }

        fn len(&self) -> usize {
            self.len
        }

        fn capacity(&self) -> usize {
            self.capacity
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            let len = self.len();
            let remaining = self.capacity() - len;

            if remaining < cnt {
                panic_advance(&TryGetError {
                    requested: cnt,
                    available: remaining,
                });
            }

            self.set_len(len + cnt);
        }

        fn set_len(&mut self, new_len: usize) {
            self.len = new_len;
        }
    }

    struct TryGetError {
        requested: usize,
        available: usize,
    }

    fn panic_advance(err: &TryGetError) {
        panic!("Requested {}, but only {} available", err.requested, err.available);
    }

    let mut buf = BufMut::new(10);
    unsafe {
        buf.advance_mut(5);
    }
    assert_eq!(buf.len(), 5);
}

#[test]
#[should_panic(expected = "Requested 6, but only 5 available")]
fn test_advance_mut_exceed_capacity() {
    struct BufMut {
        data: Vec<u8>,
        len: usize,
        capacity: usize,
    }

    impl BufMut {
        fn new(capacity: usize) -> Self {
            let data = Vec::with_capacity(capacity);
            Self { data, len: 0, capacity }
        }

        fn len(&self) -> usize {
            self.len
        }

        fn capacity(&self) -> usize {
            self.capacity
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            let len = self.len();
            let remaining = self.capacity() - len;

            if remaining < cnt {
                panic_advance(&TryGetError {
                    requested: cnt,
                    available: remaining,
                });
            }

            self.set_len(len + cnt);
        }

        fn set_len(&mut self, new_len: usize) {
            self.len = new_len;
        }
    }

    struct TryGetError {
        requested: usize,
        available: usize,
    }

    fn panic_advance(err: &TryGetError) {
        panic!("Requested {}, but only {} available", err.requested, err.available);
    }

    let mut buf = BufMut::new(5);
    unsafe {
        buf.advance_mut(6);
    }
}

