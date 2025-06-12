// Answer 0

#[test]
fn test_partial_shuffle_non_empty() {
    struct TestRng;

    impl Rng for TestRng {
        // Implement required RNG methods based on your use case
    }

    // Generating a slice of integers
    let mut slice = [1, 2, 3, 4, 5];
    let mut rng = TestRng;

    // Testing with amount less than length of the slice
    let (shuffled_part, remaining_part) = slice.partial_shuffle(&mut rng, 2);
    
    assert_eq!(shuffled_part.len(), 2);
    assert_eq!(remaining_part.len(), 3);
}

#[test]
fn test_partial_shuffle_empty() {
    struct TestRng;

    impl Rng for TestRng {
        // Implement required RNG methods based on your use case
    }

    // Edge case: empty slice
    let mut slice: Vec<i32> = Vec::new();
    let mut rng = TestRng;

    let (shuffled_part, remaining_part) = slice.partial_shuffle(&mut rng, 0);
    
    assert!(shuffled_part.is_empty());
    assert!(remaining_part.is_empty());
}

#[test]
fn test_partial_shuffle_boundary_condition() {
    struct TestRng;

    impl Rng for TestRng {
        // Implement required RNG methods based on your use case
    }

    // Edge case: full shuffle
    let mut slice = [1, 2, 3, 4, 5];
    let mut rng = TestRng;

    let (shuffled_part, remaining_part) = slice.partial_shuffle(&mut rng, 5);
    
    assert_eq!(shuffled_part.len(), 5);
    assert!(remaining_part.is_empty());
}

#[test]
#[should_panic]
fn test_partial_shuffle_panic_on_split() {
    struct TestRng;

    impl Rng for TestRng {
        // Implement required RNG methods based on your use case
    }

    // Creating a slice and intentionally invoking a panic
    let mut slice = [1, 2, 3];
    let mut rng = TestRng;

    // This should trigger a panic since amount >= length
    let _ = slice.partial_shuffle(&mut rng, 4);
}

