// Answer 0

#[test]
fn test_find_entry_not_found() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let result = table.find_entry(hasher(&2), |val| val.0 == 2);
    
    match result {
        Err(_) => {}
        _ => panic!("Expected an Err variant for a non-existing entry."),
    }
}

#[test]
fn test_find_entry_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let result = table.find_entry(hasher(&99), |val| val.0 == 99);
    
    match result {
        Err(_) => {}
        _ => panic!("Expected an Err variant when searching in an empty table."),
    }
}

#[test]
fn test_find_entry_non_matching_eq() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&1), (1, "a"), |val| hasher(&val.0));
    
    let result = table.find_entry(hasher(&1), |val| val.0 == 2);
    
    match result {
        Err(_) => {}
        _ => panic!("Expected an Err variant due to non-matching equality function."),
    }
}

