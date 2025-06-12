// Answer 0

#[test]
fn test_choose_multiple_with_insufficient_elements() {
    let mut rng = fastrand::Rng::new();
    let source = vec![1, 2]; // Only 2 elements in the source
    let amount = 5; // Requesting more elements than available
    
    let result = rng.choose_multiple(source, amount);
    
    assert_eq!(result.len(), 2); // Should return length equal to available elements
    assert_eq!(result, vec![1, 2]); // Should return all available elements
}

#[test]
fn test_choose_multiple_with_boundary_condition() {
    let mut rng = fastrand::Rng::new();
    let source = (1..=10).collect::<Vec<_>>(); // 10 elements in the source
    let amount = 3; // Requesting a reasonable amount
    
    let result = rng.choose_multiple(source.clone(), amount);
    
    assert_eq!(result.len(), 3); // Should return length equal to requested amount
    assert!(result.iter().all(|&x| source.contains(&x))); // Should return elements from source
}

#[test]
fn test_choose_multiple_with_large_capacity() {
    let mut rng = fastrand::Rng::new();
    let source = (1..=8).collect::<Vec<_>>(); // 8 elements in the source
    let amount = 2; // Requesting a small amount
    
    let result = rng.choose_multiple(source.clone(), amount);
    
    assert_eq!(result.len(), 2); // Should return length equal to requested amount
    assert!(result.iter().all(|&x| source.contains(&x))); // Should return elements from source
    // Checking the condition for reservoir capacity
    // Ensure the capacity is more than 3 times the length of the reservoir
    let mut vec = Vec::with_capacity(1); // Creating a vector to check capacity
    vec.extend(result.iter());
    assert!(vec.capacity() > 3 * vec.len()); // Validating capacity condition
}

