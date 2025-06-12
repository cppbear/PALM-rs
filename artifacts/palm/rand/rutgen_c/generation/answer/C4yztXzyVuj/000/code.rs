// Answer 0

#[test]
fn test_random_ratio_one_over() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 3 }; // Example initial value
    let mut flipper = CoinFlipper::new(rng);

    assert!(flipper.random_ratio_one_over(4));  // Test case where d is 4
    assert!(!flipper.random_ratio_one_over(1)); // Test case where d is 1

    // Test panic case
    let result = std::panic::catch_unwind(|| {
        flipper.random_ratio_one_over(0);
    });
    assert!(result.is_err());  // Ensure panic occurs on d = 0
}

#[test]
fn test_random_ratio_one_over_boundary() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 1 }; // Example initial value
    let mut flipper = CoinFlipper::new(rng);

    assert!(flipper.random_ratio_one_over(2)); // Test with d being a power of 2
    assert!(!flipper.random_ratio_one_over(8)); // Test with d greater than potential

    let result = std::panic::catch_unwind(|| {
        flipper.random_ratio_one_over(0);
    });
    assert!(result.is_err());  // Ensure panic occurs on d = 0
}

