// Answer 0

#[test]
fn test_capacity_non_empty_set() {
    struct TestHasher; // Stub for a hash builder
    let hash_builder = TestHasher;
    let mut index_set = IndexSet::with_capacity_and_hasher(10, hash_builder);
    
    index_set.map.insert(1, ()); // Add an element to the set
    assert_eq!(index_set.capacity(), 10);
}

#[test]
fn test_capacity_empty_set() {
    struct TestHasher; // Stub for a hash builder
    let hash_builder = TestHasher;
    let index_set = IndexSet::with_capacity_and_hasher(0, hash_builder);
    
    assert_eq!(index_set.capacity(), 0);
}

#[test]
fn test_capacity_after_reserve() {
    struct TestHasher; // Stub for a hash builder
    let hash_builder = TestHasher;
    let mut index_set = IndexSet::with_capacity_and_hasher(5, hash_builder);
    
    index_set.reserve(10); // Reserve space for 10 more elements
    assert_eq!(index_set.capacity(), 5); // Still should return initial capacity since it doesn't increase until actual reallocation
}

#[test]
fn test_capacity_after_truncation() {
    struct TestHasher; // Stub for a hash builder
    let hash_builder = TestHasher;
    let mut index_set = IndexSet::with_capacity_and_hasher(10, hash_builder);
    
    index_set.map.insert(1, ()); // Add one element
    index_set.truncate(0); // Truncate to zero
    assert_eq!(index_set.capacity(), 10); // Should still return the original capacity even if empty
}

#[test]
fn test_capacity_with_large_initial_value() {
    struct TestHasher; // Stub for a hash builder
    let hash_builder = TestHasher;
    let index_set = IndexSet::with_capacity_and_hasher(1_000_000, hash_builder);
    
    assert_eq!(index_set.capacity(), 1_000_000);
}

