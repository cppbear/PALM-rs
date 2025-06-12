// Answer 0

#[test]
fn test_drain_non_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    for x in 1..=5 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    
    assert!(!table.is_empty());

    let mut drained_elements: Vec<_> = table.drain().collect();
    
    assert_eq!(drained_elements.len(), 5);
    drained_elements.sort();
    assert_eq!(drained_elements, vec![1, 2, 3, 4, 5]);
    
    assert!(table.is_empty());
}

#[test]
fn test_drain_empty_table() {
    use hashbrown::HashTable;
    
    let mut table: HashTable<i32, i32> = HashTable::new();
    
    assert!(table.is_empty());

    let drained_elements: Vec<_> = table.drain().collect();
    
    assert!(drained_elements.is_empty());
}

