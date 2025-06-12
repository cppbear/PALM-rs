// Answer 0

#[test]
fn test_is_empty_on_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    let v: HashTable<_, _, _> = HashTable::new();
    assert!(v.is_empty());
}

#[test]
fn test_is_empty_after_insertion() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    let mut v: HashTable<_, _, _> = HashTable::new();
    v.insert_unique(hasher_fn(&1), 1, hasher_fn);
    assert!(!v.is_empty());
}

