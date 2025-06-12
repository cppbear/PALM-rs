// Answer 0

#[test]
fn test_choose_multiple_with_exhaustion() {
    let mut rng = Rng::with_seed(42);
    let source = vec![1, 2, 3];
    let amount = 5; // Greater than the number of elements in source

    // This should return a vector containing the available elements,
    // which in this case will be [1, 2, 3].
    let result = rng.choose_multiple(source, amount);

    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_choose_multiple_with_capacity_condition() {
    let mut rng = Rng::with_seed(42);
    let source: Vec<_> = (1..100).collect(); // 99 elements
    let amount = 30; // Taking a significant amount

    let result = rng.choose_multiple(source, amount);
    
    // Confirm the result constraints
    assert_eq!(result.len(), amount); // should equal the amount specified
    assert!(result.capacity() > 3 * result.len()); // verify the capacity constraint
}

