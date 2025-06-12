// Answer 0

#[cfg(test)]
fn fill_bytes_test() {
    struct MockRng;

    impl MockRng {
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for i in 0..dest.len() {
                dest[i] = i as u8; // Fill with sequential values for testing
            }
        }
    }

    struct TestRng {
        rng: std::cell::UnsafeCell<MockRng>,
    }

    impl TestRng {
        fn new() -> Self {
            TestRng {
                rng: std::cell::UnsafeCell::new(MockRng),
            }
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            // SAFETY: We ensure exclusive access to `rng` by not
            // allowing other mutable references while this function is in use.
            let rng = unsafe { &mut *self.rng.get() };
            rng.fill_bytes(dest);
        }
    }

    let mut test_rng = TestRng::new();
    let mut buffer = [0u8; 10];
    test_rng.fill_bytes(&mut buffer);

    assert_eq!(buffer, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_fill_bytes() {
    fill_bytes_test();
}

