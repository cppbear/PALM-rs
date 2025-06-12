// Answer 0

#[test]
fn test_retain_removes_uneven_elements() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new_in(Global);
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    
    for x in 1..=6 {
        table.insert_unique(hasher_fn(&x), x, hasher_fn);
    }
    
    table.retain(|&mut x| x % 2 == 0);
    assert_eq!(table.len(), 3);
}

#[test]
fn test_retain_empty_table() {
    let mut table: HashTable<i32, Global> = HashTable::new_in(Global);
    table.retain(|_| false);
    assert_eq!(table.len(), 0);
}

#[test]
fn test_retain_removes_all_elements() {
    let mut table = HashTable::new_in(Global);
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    
    for x in 1..=5 {
        table.insert_unique(hasher_fn(&x), x, hasher_fn);
    }
    
    table.retain(|_| false);
    assert_eq!(table.len(), 0);
}

#[test]
fn test_retain_keeps_all_elements() {
    let mut table = HashTable::new_in(Global);
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    
    for x in 1..=5 {
        table.insert_unique(hasher_fn(&x), x, hasher_fn);
    }
    
    table.retain(|_| true);
    assert_eq!(table.len(), 5);
}

