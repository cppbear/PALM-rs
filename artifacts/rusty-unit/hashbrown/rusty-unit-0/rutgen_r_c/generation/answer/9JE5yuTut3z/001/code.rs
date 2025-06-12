// Answer 0

#[test]
fn test_is_empty_on_new_hash_table() {
    use hashbrown::{HashTable, Global};
    
    let table: HashTable<i32, Global> = HashTable::new_in(Global);
    assert!(table.is_empty());
}

#[test]
fn test_is_empty_after_insertion() {
    use hashbrown::{HashTable, Global};
    use std::hash::BuildHasher;
    use std::collections::hash_map::DefaultHasher;

    let hasher = |val: &i32| {
        let mut s = DefaultHasher::new();
        val.hash(&mut s);
        s.finish()
    };

    let mut table: HashTable<i32, Global> = HashTable::new_in(Global);
    assert!(table.is_empty());
    table.insert_unique(hasher(&1), 1, hasher);
    assert!(!table.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    use hashbrown::{HashTable, Global};
    use std::hash::BuildHasher;
    use std::collections::hash_map::DefaultHasher;

    let hasher = |val: &i32| {
        let mut s = DefaultHasher::new();
        val.hash(&mut s);
        s.finish()
    };

    let mut table: HashTable<i32, Global> = HashTable::new_in(Global);
    table.insert_unique(hasher(&1), 1, hasher);
    assert!(!table.is_empty());
    table.clear();
    assert!(table.is_empty());
} 

#[test]
fn test_is_empty_on_shrink_to_fit() {
    use hashbrown::{HashTable, Global};
    use std::hash::BuildHasher;
    use std::collections::hash_map::DefaultHasher;

    let hasher = |val: &i32| {
        let mut s = DefaultHasher::new();
        val.hash(&mut s);
        s.finish()
    };

    let mut table: HashTable<i32, Global> = HashTable::new_in(Global);
    table.insert_unique(hasher(&1), 1, hasher);
    assert!(!table.is_empty());
    table.shrink_to_fit(hasher);
    assert!(!table.is_empty());
}

