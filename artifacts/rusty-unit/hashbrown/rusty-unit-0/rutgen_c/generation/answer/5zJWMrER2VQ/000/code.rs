// Answer 0

#[test]
fn test_drain_non_empty() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new_in(DefaultHashBuilder::default());
    let hasher = |val: &_| val.hash();
    
    for x in 1..=3 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    assert!(!table.is_empty());

    let drained_elements: Vec<_> = table.drain().collect();
    
    assert_eq!(drained_elements.len(), 3);
    assert!(table.is_empty());
}

#[test]
fn test_drain_empty() {
    use hashbrown::HashTable;

    let mut table: HashTable<i32> = HashTable::new_in(Global::default());
    assert!(table.is_empty());

    let drained_elements: Vec<_> = table.drain().collect();
    
    assert!(drained_elements.is_empty());
    assert!(table.is_empty());
}

