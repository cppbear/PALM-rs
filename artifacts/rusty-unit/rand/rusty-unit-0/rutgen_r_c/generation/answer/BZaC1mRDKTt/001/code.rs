// Answer 0

#[test]
fn test_random_ratio_true_case() {
    struct MockRng {
        value: u32,
    }
    
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b11111111 }; // Simulating a case where flip_c_heads yields true
    let mut flipper = CoinFlipper::new(rng);
    
    // n = 2, d = 4 (n < d)
    assert_eq!(flipper.random_ratio(2, 4), true);
}

#[test]
fn test_random_ratio_flip_heads() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b11111111111111111111111111111111 }; // All heads for multiple flips
    let mut flipper = CoinFlipper::new(rng);

    // n = 1, d = 4 (n < d) and we ensure multiple heads
    assert_eq!(flipper.random_ratio(1, 4), true);
}

#[test]
fn test_random_ratio_boundary_case_equal() {
    struct MockRng {
        value: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0b11111111 }; // Simulating a case where flip_c_heads yields true
    let mut flipper = CoinFlipper::new(rng);

    // n = 4, d = 4 (n == d)
    assert_eq!(flipper.random_ratio(4, 4), true);
}

