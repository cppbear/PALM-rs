// Answer 0

#[test]
fn test_hashmap_reserve() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    
    // Initial capacity should be 0
    assert_eq!(map.capacity(), 0);
    
    // Reserve space for 10 additional elements
    map.reserve(10);
    
    // Check if capacity is at least 10
    assert!(map.capacity() >= 10);
}

#[test]
#[should_panic] // Testing the panic when exceeding usize::MAX
fn test_hashmap_reserve_panic() {
    use hashbrown::HashMap;

    let mut map: HashMap<u64, u64> = HashMap::new();
    
    // Reserve space exceeding isize::MAX
    map.reserve(usize::MAX);
}

