// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Test inserting into a vacant entry
    let entry_result = table
        .entry(hasher(&"vacant_key"), |&x| x == "vacant_key", hasher)
        .or_insert("default_value");

    // Verify that the entry was inserted
    assert_eq!(entry_result, &mut "default_value");
    assert!(table
        .find(hasher(&"vacant_key"), |&x| x == "vacant_key")
        .is_some());
    assert_eq!(table.len(), 1);
}

#[test]
fn test_or_insert_existing_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert an initial value
    table
        .entry(hasher(&"existing_key"), |&x| x == "existing_key", hasher)
        .or_insert("initial_value");

    // Attempt to insert a different value into the occupied entry
    let entry_result = table
        .entry(hasher(&"existing_key"), |&x| x == "existing_key", hasher)
        .or_insert("new_value");

    // Verify that the entry still holds the original value
    assert_eq!(entry_result, &mut "initial_value");
    assert!(table
        .find(hasher(&"existing_key"), |&x| x == "existing_key")
        .is_some());
    assert_eq!(table.len(), 1);
}

