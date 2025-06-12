// Answer 0

#[test]
fn test_with_capacity_in_zero_capacity() {
    use bumpalo::Bump;
    use hashbrown::HashTable;

    let bump = Bump::new();
    let table = HashTable::with_capacity_in(0, &bump);

    assert_eq!(table.len(), 0);
    assert!(table.capacity() == 0);
}

#[test]
fn test_with_capacity_in_non_zero_capacity() {
    use bumpalo::Bump;
    use hashbrown::HashTable;
    use std::hash::BuildHasher;
    use ahash::RandomState; // Using ahash as an example allocator, ensure to include it in your Cargo.toml

    let bump = Bump::new();
    let mut table = HashTable::with_capacity_in(5, &bump);
    
    let hasher = RandomState::new();
    let hash_fn = |val: &str| hasher.build_hasher().hash_one(val);
    
    assert_eq!(table.len(), 0);
    let initial_capacity = table.capacity();
    assert!(initial_capacity >= 5);

    table.insert_unique(hash_fn("One"), "One", hash_fn);
    table.insert_unique(hash_fn("Two"), "Two", hash_fn);
    table.insert_unique(hash_fn("Three"), "Three", hash_fn);
    table.insert_unique(hash_fn("Four"), "Four", hash_fn);
    table.insert_unique(hash_fn("Five"), "Five", hash_fn);

    assert_eq!(table.len(), 5);
    assert_eq!(table.capacity(), initial_capacity);
}

