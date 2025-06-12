// Answer 0

#[test]
fn test_is_subset_equal_sets() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    #[derive(Eq, PartialEq, Hash)]
    struct Item(i32);
    
    let hasher = DefaultHasher::new();
    let mut set1 = IndexSet::with_capacity_and_hasher(3, hasher);
    let mut set2 = IndexSet::with_capacity_and_hasher(3, hasher);
    
    set1.insert(Item(1));
    set1.insert(Item(2));
    set1.insert(Item(3));
    
    set2.insert(Item(1));
    set2.insert(Item(2));
    set2.insert(Item(3));
    
    assert!(set1.is_subset(&set2));
}

#[test]
fn test_is_subset_subset_sets() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    #[derive(Eq, PartialEq, Hash)]
    struct Item(i32);
    
    let hasher = DefaultHasher::new();
    let mut set1 = IndexSet::with_capacity_and_hasher(2, hasher);
    let mut set2 = IndexSet::with_capacity_and_hasher(3, hasher);
    
    set1.insert(Item(1));
    set1.insert(Item(2));
    
    set2.insert(Item(1));
    set2.insert(Item(2));
    set2.insert(Item(3));
    
    assert!(set1.is_subset(&set2));
}

#[test]
fn test_is_subset_empty_set() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    #[derive(Eq, PartialEq, Hash)]
    struct Item(i32);
    
    let hasher = DefaultHasher::new();
    let mut set1 = IndexSet::with_capacity_and_hasher(0, hasher);
    let mut set2 = IndexSet::with_capacity_and_hasher(3, hasher);
    
    set2.insert(Item(1));
    set2.insert(Item(2));
    
    assert!(set1.is_subset(&set2));
}

#[test]
fn test_is_subset_different_sets() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    #[derive(Eq, PartialEq, Hash)]
    struct Item(i32);
    
    let hasher = DefaultHasher::new();
    let mut set1 = IndexSet::with_capacity_and_hasher(3, hasher);
    let mut set2 = IndexSet::with_capacity_and_hasher(3, hasher);
    
    set1.insert(Item(1));
    set1.insert(Item(2));
    
    set2.insert(Item(1));
    set2.insert(Item(2));
    set2.insert(Item(3));
    
    assert!(set1.is_subset(&set2));
}

