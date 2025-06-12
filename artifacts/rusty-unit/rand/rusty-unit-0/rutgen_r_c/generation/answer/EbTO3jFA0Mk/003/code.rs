// Answer 0

#[test]
fn test_sample_array_success() {
    struct MockRng {
        counter: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let result = self.counter % (range.end - range.start) + range.start;
            self.counter += 1;
            result
        }
    }

    let mut rng = MockRng { counter: 0 };
    let len = 5;
    const N: usize = 5;

    let result = sample_array(&mut rng, len);
    assert!(result.is_some());
    
    let indices = result.unwrap();
    let unique_indices: std::collections::HashSet<_> = indices.iter().cloned().collect();
    assert_eq!(unique_indices.len(), N);
    assert!(unique_indices.iter().all(|&x| x < len));
}

#[test]
fn test_sample_array_too_large_n() {
    struct MockRng {}

    impl Rng for MockRng {
        fn random_range(&mut self, _: std::ops::Range<usize>) -> usize {
            // This function won't be called because of the early return
            0
        }
    }

    let mut rng = MockRng {};
    let len = 3;
    const N: usize = 4;

    let result = sample_array(&mut rng, len);
    assert!(result.is_none());
}

#[test]
fn test_sample_array_boundaries() {
    struct MockRng {
        counter: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let result = self.counter % (range.end - range.start) + range.start;
            self.counter += 1;
            result
        }
    }

    let mut rng = MockRng { counter: 0 };
    let len = 10;
    const N: usize = 1;

    let result = sample_array(&mut rng, len);
    assert!(result.is_some());

    let indices = result.unwrap();
    assert_eq!(indices[0], 0); // Should always return 0 since N == 1 and the range is generated from 0
}

#[test]
fn test_sample_array_no_repeats() {
    struct MockRng {
        counter: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let result = self.counter % (range.end - range.start) + range.start;
            self.counter += 1;
            result
        }
    }

    let mut rng = MockRng { counter: 0 };
    let len = 6;
    const N: usize = 4;

    let result = sample_array(&mut rng, len).expect("Should return Some");
    let unique_indices: std::collections::HashSet<_> = result.iter().cloned().collect();
    assert_eq!(unique_indices.len(), N);
}

