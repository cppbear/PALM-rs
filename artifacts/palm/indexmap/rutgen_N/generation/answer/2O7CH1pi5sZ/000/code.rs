// Answer 0

#[test]
fn test_with_capacity_and_hasher_zero_capacity() {
    use indexmap::IndexMap;
    use std::collections::hash_map::DefaultHasher;
    
    struct IndexSet<S> {
        map: IndexMap<u32, (), S>,
    }
    
    let hasher = DefaultHasher::new();
    let set: IndexSet<DefaultHasher> = with_capacity_and_hasher(0, hasher);
    
    assert_eq!(set.map.len(), 0);
}

#[test]
fn test_with_capacity_and_hasher_non_zero_capacity() {
    use indexmap::IndexMap;
    use std::collections::hash_map::DefaultHasher;
    
    struct IndexSet<S> {
        map: IndexMap<u32, (), S>,
    }
    
    let hasher = DefaultHasher::new();
    let set: IndexSet<DefaultHasher> = with_capacity_and_hasher(5, hasher);
    
    assert_eq!(set.map.capacity(), 5);
    assert_eq!(set.map.len(), 0);
}

#[test]
fn test_with_capacity_and_hasher_large_capacity() {
    use indexmap::IndexMap;
    use std::collections::hash_map::DefaultHasher;
    
    struct IndexSet<S> {
        map: IndexMap<u32, (), S>,
    }
    
    let hasher = DefaultHasher::new();
    let set: IndexSet<DefaultHasher> = with_capacity_and_hasher(1000, hasher);
    
    assert_eq!(set.map.capacity(), 1000);
}

