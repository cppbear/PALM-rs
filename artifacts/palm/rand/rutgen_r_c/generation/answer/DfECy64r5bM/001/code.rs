// Answer 0

#[test]
fn test_shuffle_single_element_slice() {
    struct RngStub;

    impl Rng for RngStub {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            // Return a fixed value for deterministic behavior in tests
            0
        }
    }

    let mut rng = RngStub;
    let mut single_element_slice = [42];
    
    single_element_slice.shuffle(&mut rng);
    
    // The slice should remain unchanged
    assert_eq!(single_element_slice, [42]);
}

#[test]
fn test_shuffle_empty_slice() {
    struct RngStub;

    impl Rng for RngStub {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            // Return a fixed value for deterministic behavior in tests
            0
        }
    }

    let mut rng = RngStub;
    let mut empty_slice: &mut [i32] = &mut [];
    
    empty_slice.shuffle(&mut rng);
    
    // The slice should remain unchanged
    assert_eq!(empty_slice.len(), 0);
}

