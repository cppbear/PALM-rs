// Answer 0

#[test]
fn test_find_mut_update_existing_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert_unique(hasher(&1), (1, "a"), |val| hasher(&val.0));
    
    if let Some(val) = table.find_mut(hasher(&1), |val| val.0 == 1) {
        val.1 = "b";
    }
    
    assert_eq!(table.find(hasher(&1), |val| val.0 == 1), Some(&(1, "b")));
}

#[test]
fn test_find_mut_no_entry_found() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert_unique(hasher(&1), (1, "a"), |val| hasher(&val.0));

    let result = table.find_mut(hasher(&2), |val| val.0 == 2);
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_find_mut_panic_on_invalid_mutation() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    table.insert_unique(hasher(&1), (1, "a"), |val| hasher(&val.0));
    
    // This will cause an invalid mutation since we're expecting a different hash
    let _ = table.find_mut(hasher(&2), |val| val.0 == 1);
}

