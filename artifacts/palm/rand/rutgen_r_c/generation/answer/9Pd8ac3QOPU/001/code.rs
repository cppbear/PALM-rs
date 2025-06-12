// Answer 0

#[test]
fn test_next_index_initial_state() {
    struct MockRng;

    impl RngCore for MockRng {
        fn random_range(&mut self, range: std::ops::Range<u32>) -> u32 {
            range.start // Always return the lower bound for predictability
        }
    }

    let mut rng = MockRng;
    let mut increasing_uniform = IncreasingUniform::new(rng, 0);

    let index_0 = increasing_uniform.next_index();
    assert_eq!(index_0, 0); // First call should return 0
}

#[test]
fn test_next_index_increment_n() {
    struct MockRng;

    impl RngCore for MockRng {
        fn random_range(&mut self, range: std::ops::Range<u32>) -> u32 {
            range.start // Always return the lower bound for predictability
        }
    }

    let mut rng = MockRng;
    let mut increasing_uniform = IncreasingUniform::new(rng, 1);

    let index_1 = increasing_uniform.next_index();
    assert_eq!(index_1, 0); // Second call should return 0

    let index_2 = increasing_uniform.next_index();
    assert_eq!(index_2, 1); // Third call should return 1 (as n increments)
}

#[test]
fn test_next_index_multiple_calls() {
    struct MockRng;

    impl RngCore for MockRng {
        fn random_range(&mut self, range: std::ops::Range<u32>) -> u32 {
            // Return a number within the range for testing, we simulate randomness here,
            // for simplicity we will keep it deterministic.
            if range.start < range.end {
                range.start + 1 // Always pick the second number
            } else {
                range.start
            }
        }
    }

    let mut rng = MockRng;
    let mut increasing_uniform = IncreasingUniform::new(rng, 5);

    let index_1 = increasing_uniform.next_index();
    assert_eq!(index_1, 0); // Should return 0

    let index_2 = increasing_uniform.next_index();
    assert_eq!(index_2, 1); // Should return 1

    let index_3 = increasing_uniform.next_index();
    assert_eq!(index_3, 2); // Should return 2

    let index_4 = increasing_uniform.next_index();
    assert_eq!(index_4, 3); // Should return 3

    let index_5 = increasing_uniform.next_index();
    assert_eq!(index_5, 4); // Should return 4
}

#[should_panic]
#[test]
fn test_next_index_panics_on_overflow() {
    struct MockRng;

    impl RngCore for MockRng {
        fn random_range(&mut self, _range: std::ops::Range<u32>) -> u32 {
            0 // Not relevant for this test
        }
    }

    let mut rng = MockRng;
    let mut increasing_uniform = IncreasingUniform::new(rng, u32::MAX);
    let _ = increasing_uniform.next_index(); // This should trigger a panic
}

