// Answer 0

#[test]
fn test_shuffle_multiple_elements() {
    struct TestRng {
        value: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.value % range.len()
        }
    }

    let mut rng = TestRng { value: 2 };
    let mut slice = [1, 2, 3, 4, 5];

    // Calling shuffle on a slice with more than one element
    slice.shuffle(&mut rng);

    // Check that the slice has not been completely unaltered,
    // since we expect a shuffle to mix elements
    assert!(slice != [1, 2, 3, 4, 5]);
}

#[test]
fn test_shuffle_empty_slice() {
    struct TestRng {
        value: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.value % range.len()
        }
    }

    let mut rng = TestRng { value: 0 };
    let mut slice: [i32; 0] = [];

    // Calling shuffle on an empty slice
    slice.shuffle(&mut rng);

    // An empty slice should remain empty
    assert!(slice.is_empty());
}

#[test]
fn test_shuffle_single_element() {
    struct TestRng {
        value: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.value % range.len()
        }
    }

    let mut rng = TestRng { value: 1 };
    let mut slice = [10];

    // Calling shuffle on a slice with a single element
    slice.shuffle(&mut rng);

    // A single-element slice should remain unchanged
    assert_eq!(slice, [10]);
}

