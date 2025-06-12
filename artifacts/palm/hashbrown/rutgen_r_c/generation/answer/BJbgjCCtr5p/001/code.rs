// Answer 0

#[test]
fn test_symmetric_difference_basic() {
    // Create two basic HashSets
    let mut set_a: HashSet<_> = HashSet::new();
    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);
    
    let mut set_b: HashSet<_> = HashSet::new();
    set_b.insert(4);
    set_b.insert(2);
    set_b.insert(3);
    set_b.insert(4);
    
    // Calculate symmetric difference
    let diff = set_a.symmetric_difference(&set_b);
    
    let result: Vec<_> = diff.iter.collect();
    
    // Verify that the result contains 1 and 4
    assert!(result.contains(&1));
    assert!(result.contains(&4));
    assert_eq!(result.len(), 2);
}

#[test]
fn test_symmetric_difference_empty_sets() {
    // Create two empty HashSets
    let set_a: HashSet<i32> = HashSet::new();
    let set_b: HashSet<i32> = HashSet::new();
    
    // Calculate symmetric difference
    let diff = set_a.symmetric_difference(&set_b);
    
    let result: Vec<_> = diff.iter.collect();
    
    // Verify the result is empty
    assert!(result.is_empty());
}

#[test]
fn test_symmetric_difference_identical_sets() {
    // Create two identical HashSets
    let mut set_a: HashSet<_> = HashSet::new();
    set_a.insert(1);
    set_a.insert(2);
    
    let set_b = set_a.clone();
    
    // Calculate symmetric difference
    let diff = set_a.symmetric_difference(&set_b);
    
    let result: Vec<_> = diff.iter.collect();
    
    // Verify the result is empty
    assert!(result.is_empty());
}

#[test]
fn test_symmetric_difference_one_empty_one_non_empty() {
    // Create an empty HashSet and a non-empty HashSet
    let set_a: HashSet<i32> = HashSet::new();
    
    let mut set_b: HashSet<_> = HashSet::new();
    set_b.insert(5);
    set_b.insert(6);
    
    // Calculate symmetric difference
    let diff = set_a.symmetric_difference(&set_b);
    
    let result: Vec<_> = diff.iter.collect();
    
    // Verify that the result contains 5 and 6
    assert!(result.contains(&5));
    assert!(result.contains(&6));
    assert_eq!(result.len(), 2);
}

