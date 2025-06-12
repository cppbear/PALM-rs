// Answer 0

#[test]
fn test_or_insert_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;
    
    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    // Existing key for testing Entry::Occupied
    table.insert_unique(hasher(&"exists"), "exists", hasher);
    
    // Retrieve occupied entry
    let entry = table.entry(hasher(&"exists"), |&x| x == "exists", hasher);
    match entry {
        Entry::Occupied(occupied_entry) => {
            let value = occupied_entry.get();
            assert_eq!(value, "exists");
        }
        _ => panic!("Expected Entry::Occupied"),
    }
}

#[test]
fn test_or_insert_vacant_entry_inserts_value() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;
    
    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    // Nonexistent key for testing Entry::Vacant
    let entry = table.entry(hasher(&"new_key"), |&x| x == "new_key", hasher);
    let occupied_entry = entry.or_insert("new_key");
    
    assert!(table.find(hasher(&"new_key"), |&x| x == "new_key").is_some());
    assert_eq!(table.len(), 1);
    assert_eq!(occupied_entry.get(), &"new_key");
}

