// Answer 0

#[test]
fn or_insert_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder, Entry};
    use std::hash::{BuildHasher, Hash};

    // Initialize a new hash table
    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    // Test inserting a value when the entry is vacant
    let vacant_entry = table.entry(hasher_fn(&"vacant_key"), |&x| x == "vacant_key", hasher_fn);
    let occupied_entry = vacant_entry.or_insert("vacant_value");

    // Ensure the entry is now occupied
    assert!(matches!(occupied_entry, Entry::Occupied(_)));
    assert!(table.find(hasher_fn(&"vacant_key"), |&x| x == "vacant_key").is_some());
    assert_eq!(table.len(), 1);
}

#[test]
fn or_insert_existing_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder, Entry};
    use std::hash::{BuildHasher, Hash};

    // Initialize a new hash table and insert an initial value
    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher_fn(&"existing_key"), "existing_value", hasher_fn);

    // Test inserting a value when the entry already exists
    let occupied_entry = table.entry(hasher_fn(&"existing_key"), |&x| x == "existing_key", hasher_fn);
    let new_occupied_entry = occupied_entry.or_insert("new_value");

    // Ensure the entry remains occupied and the value is not overwritten
    assert!(matches!(new_occupied_entry, Entry::Occupied(_)));
    assert_eq!(table.len(), 1);
    assert_eq!(table.find(hasher_fn(&"existing_key"), |&x| x == "existing_key").unwrap(), "existing_value");
}

