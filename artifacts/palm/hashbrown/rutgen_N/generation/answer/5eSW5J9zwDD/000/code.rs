// Answer 0

#[test]
fn test_iter_mut() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;
    
    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.insert_unique(hasher(&3), 3, hasher);
    
    for val in table.iter_mut() {
        *val *= 2;
    }
    
    assert_eq!(table.len(), 3);
    let mut vec: Vec<i32> = Vec::new();
    
    for val in &table {
        vec.push(*val);
    }
    
    vec.sort_unstable();
    assert_eq!(vec, [2, 4, 6]);
    
    assert_eq!(table.len(), 3);
}

#[test]
fn test_iter_mut_empty() {
    use hashbrown::HashTable;

    let mut table: HashTable<i32, i32> = HashTable::new();
    
    for val in table.iter_mut() {
        panic!("Iterator should not yield any values for an empty table.");
    }
    
    assert_eq!(table.len(), 0);
}

#[test]
#[should_panic(expected = "attempt to mutate an item during iteration")]
fn test_iter_mut_panic() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert_unique(hasher(&1), 1, hasher);
    
    for val in table.iter_mut() {
        table.insert_unique(hasher(&2), 2, hasher);
        *val *= 2;  // This should panic due to concurrent mutation.
    }
}

