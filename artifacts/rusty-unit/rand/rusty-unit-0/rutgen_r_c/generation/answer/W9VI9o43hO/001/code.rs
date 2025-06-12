// Answer 0

#[test]
fn test_next_u64() {
    // Helper struct to fulfill the Rng trait requirements
    struct TestRng {
        counter: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.counter as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.counter = self.counter.wrapping_add(1);
            self.counter
        }

        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            // No-op for simplicity
        }
    }

    // Initialize TestRng with a starting value
    let mut test_rng = TestRng { counter: 0 };

    // Create an instance of SmallRng containing the test RNG
    let mut small_rng = SmallRng(test_rng);

    // Test the next_u64 function
    assert_eq!(small_rng.next_u64(), 1);
    assert_eq!(small_rng.next_u64(), 2);
    assert_eq!(small_rng.next_u64(), 3);
}

#[test]
fn test_next_u64_max_value() {
    // Same helper struct as above for Rng trait
    struct TestRng {
        counter: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.counter as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.counter
        }

        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            // No-op for simplicity
        }
    }

    // Initialize TestRng to the maximum value for u64
    let mut test_rng = TestRng { counter: u64::MAX };

    // Create an instance of SmallRng containing the test RNG
    let mut small_rng = SmallRng(test_rng);

    // Test the next_u64 function, should not panic and should correctly return the max value
    assert_eq!(small_rng.next_u64(), u64::MAX);
}

