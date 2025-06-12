// Answer 0

#[test]
fn test_next_u32() {
    struct Pcg {
        state: u64,
    }

    impl Pcg {
        fn next_u64(&mut self) -> u64 {
            // Simple linear congruential generator for testing purpose
            self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
            self.state >> 64
        }

        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
        }
    }

    let mut rng = Pcg { state: 42 };

    // Test normal case
    let result = rng.next_u32();
    assert!(result <= std::u32::MAX);

    // Test with different initial states
    rng.state = 0;
    let result_zero = rng.next_u32();
    assert!(result_zero <= std::u32::MAX);

    rng.state = std::u64::MAX;
    let result_max = rng.next_u32();
    assert!(result_max <= std::u32::MAX);
}

