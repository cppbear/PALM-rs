// Answer 0

#[test]
fn test_next_u64() {
    struct TestRng {
        seed: u64,
    }

    impl TestRng {
        fn next_u64(&mut self) -> u64 {
            self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            self.seed
        }
    }

    let mut rng = TestRng { seed: 0 };

    let first_value = rng.next_u64();
    let second_value = rng.next_u64();

    assert!(first_value != second_value, "Random values should not be the same.");
}

#[test]
fn test_next_u64_boundary() {
    struct BoundaryRng {
        state: u64,
    }

    impl BoundaryRng {
        fn next_u64(&mut self) -> u64 {
            self.state = self.state.wrapping_add(1);
            self.state
        }
    }

    let mut rng = BoundaryRng { state: u64::MAX };

    let value = rng.next_u64();
    assert_eq!(value, 0, "The next value should wrap around to zero at the boundary.");
}

