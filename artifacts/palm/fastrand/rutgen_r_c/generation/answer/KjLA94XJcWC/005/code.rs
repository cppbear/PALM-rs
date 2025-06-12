// Answer 0

#[test]
fn test_choose_multiple_insufficient_elements() {
    struct MockRng {
        seed: u64,
    }

    impl MockRng {
        fn usize(&mut self, range: core::ops::Range<usize>) -> usize {
            range.start // Just returning start for mock purposes
        }
        fn new(seed: u64) -> Self {
            MockRng { seed }
        }
    }

    let mut rng = MockRng::new(42);
    let source = vec![1, 2]; // Source has only 2 elements
    let amount = 5; // Requesting 5 elements

    let result = rng.choose_multiple(source, amount);
    assert_eq!(result, vec![1, 2]); // Should return available elements
}

#[test]
fn test_choose_multiple_capacity_reallocation() {
    struct MockRng {
        seed: u64,
    }

    impl MockRng {
        fn usize(&mut self, range: core::ops::Range<usize>) -> usize {
            range.start // Just returning start for mock purposes
        }
        fn new(seed: u64) -> Self {
            MockRng { seed }
        }
    }

    let mut rng = MockRng::new(42);
    let source = vec![1, 2, 3, 4, 5, 6]; // Source has 6 elements
    let amount = 2; // Requesting 2 elements

    let result = rng.choose_multiple(source.clone(), amount);
    assert_eq!(result.len(), 2); // Should return 2 elements
    assert_eq!(result, vec![1, 2]); // Should return first two elements

    assert_eq!(result.capacity(), 2); // Capacity should be equal to the number of elements used
}

