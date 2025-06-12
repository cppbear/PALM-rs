// Answer 0

#[test]
fn test_find_entry_with_existing_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert_unique(hasher(&1), (1, "a"), |val| hasher(&val.0));
    let result = table.find_entry(hasher(&1), |val| val.0 == 1);
    
    assert!(result.is_ok());
}

#[test]
fn test_find_entry_with_non_existing_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let result = table.find_entry(hasher(&1), |val| val.0 == 1);
    
    assert!(result.is_err());
}

#[test]
fn test_find_entry_and_remove_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert_unique(hasher(&1), (1, "a"), |val| hasher(&val.0));
    if let Ok(entry) = table.find_entry(hasher(&1), |val| val.0 == 1) {
        entry.remove();
    }
    assert_eq!(table.find(hasher(&1), |val| val.0 == 1), None);
}

#[should_panic]
#[test]
fn test_find_entry_with_panic_on_remove_after_absent() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let entry = table.find_entry(hasher(&1), |val| val.0 == 1);
    
    if let Ok(entry) = entry {
        entry.remove();
        entry.remove(); // This will trigger a panic because the entry is already removed.
    }
}

