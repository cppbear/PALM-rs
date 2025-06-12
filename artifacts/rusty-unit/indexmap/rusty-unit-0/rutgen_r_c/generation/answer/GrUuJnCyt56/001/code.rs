// Answer 0

#[test]
fn test_clear_empty_set() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(0, std::collections::hash_map::RandomState::new());
    index_set.clear();
    assert_eq!(index_set.len(), 0);
}

#[test]
fn test_clear_non_empty_set() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    index_set.reserve(5);
    for i in 0..5 {
        index_set.map.insert(i, ());
    }
    assert_eq!(index_set.len(), 5);
    index_set.clear();
    assert_eq!(index_set.len(), 0);
}

#[test]
fn test_clear_after_reserve() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(5, std::collections::hash_map::RandomState::new());
    index_set.reserve(10);
    assert!(index_set.capacity() >= 10);
    index_set.clear();
    assert_eq!(index_set.len(), 0);
}

#[test]
fn test_clear_on_large_set() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(1000, std::collections::hash_map::RandomState::new());
    for i in 0..1000 {
        index_set.map.insert(i, ());
    }
    assert_eq!(index_set.len(), 1000);
    index_set.clear();
    assert_eq!(index_set.len(), 0);
}

#[test]
fn test_clear_preserves_capacity() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    index_set.map.insert(1, ());
    index_set.map.insert(2, ());
    assert_eq!(index_set.capacity(), 10);
    index_set.clear();
    assert_eq!(index_set.capacity(), 10);
}

