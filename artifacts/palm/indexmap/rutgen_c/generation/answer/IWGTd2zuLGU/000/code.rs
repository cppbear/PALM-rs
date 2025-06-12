// Answer 0

#[test]
fn test_shrink_to_zero_capacity() {
    struct TestKeys;
    struct TestValues;

    let mut index_map = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    index_map.core.entries.push(TestValues);
    
    index_map.shrink_to(0);
    
    assert_eq!(index_map.core.capacity(), 0);
    assert_eq!(index_map.core.len(), 0);
}

#[test]
fn test_shrink_to_non_zero_capacity() {
    struct TestKeys;
    struct TestValues;

    let mut index_map = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    index_map.core.entries.push(TestValues);
    index_map.core.entries.push(TestValues);
    
    index_map.shrink_to(1);
    
    assert_eq!(index_map.core.capacity(), 10);
    assert_eq!(index_map.core.len(), 1);
}

#[test]
fn test_shrink_to_greater_than_size() {
    struct TestKeys;
    struct TestValues;

    let mut index_map = IndexMap::with_capacity_and_hasher(3, RandomState::new());
    index_map.core.entries.push(TestValues);
    
    index_map.shrink_to(2);
    
    assert_eq!(index_map.core.capacity(), 3);
    assert_eq!(index_map.core.len(), 1);
}

