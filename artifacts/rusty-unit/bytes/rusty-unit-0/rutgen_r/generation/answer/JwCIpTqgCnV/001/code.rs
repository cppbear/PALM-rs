// Answer 0

#[test]
fn test_advance_mut_panics_when_remaining_less_than_cnt() {
    struct TestBuf {
        len: usize,
        capacity: usize,
    }

    impl TestBuf {
        fn new(capacity: usize) -> Self {
            Self { len: 0, capacity }
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
                panic!("Requested {} but available {}", cnt, remaining); // Simulating panic_advance
            }

            self.len += cnt; // Equivalent to self.set_len(len + cnt)
        }
    }

    let mut buf = TestBuf::new(5); // capacity is 5
    buf.len = 4; // current len is 4

    let cnt = 3; // requested count is 3, remaining capacity is 1 (5 - 4)
    
    let result = std::panic::catch_unwind(|| {
        unsafe { buf.advance_mut(cnt) }
    });

    assert!(result.is_err()); // We expect a panic
}

