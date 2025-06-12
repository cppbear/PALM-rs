// Answer 0

#[test]
fn test_advance_mut_with_exact_remaining_capacity() {
    struct Buffer {
        data: Vec<u8>,
        len: usize,
        capacity: usize,
    }

    impl Buffer {
        fn new(capacity: usize) -> Self {
            Self {
                data: Vec::with_capacity(capacity),
                len: 0,
                capacity,
            }
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

    fn panic_advance(error: &TryGetError) {
        panic!("Requested: {}, Available: {}", error.requested, error.available);
    }

    let capacity = 10;
    let mut buffer = Buffer::new(capacity);
    
    // Force the length to be maxed out so that remaining == cnt
    let cnt = buffer.capacity() - buffer.len(); // remaining == cnt
    unsafe {
        buffer.advance_mut(cnt);
    }
    
    assert_eq!(buffer.len(), capacity);
}

