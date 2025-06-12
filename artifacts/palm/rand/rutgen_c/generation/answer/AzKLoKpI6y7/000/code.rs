// Answer 0

#[test]
fn test_partial_shuffle_empty_slice() {
    let mut slice: &mut [i32] = &mut [];
    let mut rng = MockRng::new();
    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, 0);
    assert_eq!(shuffled.len(), 0);
    assert_eq!(remaining.len(), 0);
}

#[test]
fn test_partial_shuffle_single_element() {
    let mut slice: &mut [i32] = &mut [42];
    let mut rng = MockRng::new();
    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, 1);
    assert_eq!(shuffled.len(), 1);
    assert_eq!(remaining.len(), 0);
    assert_eq!(shuffled[0], 42);
}

#[test]
fn test_partial_shuffle_multiple_elements() {
    let mut slice: &mut [i32] = &mut [1, 2, 3, 4, 5];
    let mut rng = MockRng::new();
    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, 3);
    assert_eq!(shuffled.len(), 3);
    assert_eq!(remaining.len(), 2);
    assert!(remaining.contains(&&1) || remaining.contains(&&2) || remaining.contains(&&3) || remaining.contains(&&4) || remaining.contains(&&5));
}

#[test]
fn test_partial_shuffle_randomness() {
    let mut slice: &mut [i32] = &mut [1, 2, 3, 4, 5];
    let mut rng = MockRng::new();
    let initial_slice = slice.to_vec();
    let (shuffled, remaining) = slice.partial_shuffle(&mut rng, 3);
    assert_ne!(initial_slice, slice);
    assert_eq!(shuffled.len(), 3);
    assert_eq!(remaining.len(), 2);
}

// Mock RNG for tests
struct MockRng {
    counter: usize,
}

impl MockRng {
    fn new() -> Self {
        MockRng { counter: 0 }
    }
}

impl Rng for MockRng {
    fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
        let result = self.counter % (range.end - range.start) + range.start;
        self.counter += 1;
        result
    }
}

