// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct TestAllocator;

    // Initialize a HashTable and Insert a key-value pair
    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert an initial value
    table.insert_unique(hasher(&"key1"), "original_value", hasher);

    // Get the entry for "key1", which would be occupied
    let entry = table
        .entry(hasher(&"key1"), |&x| x == "key1", hasher)
        .insert("new_value");

    // Assert that the value has been updated
    assert_eq!(entry.get(), &"new_value");
}

#[test]
fn test_insert_update_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct TestAllocator;

    // Initialize a HashTable and Insert a key-value pair
    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert an initial value
    table.insert_unique(hasher(&"key2"), "initial_value", hasher);

    // Get the entry for "key2", which would be occupied
    let entry = table
        .entry(hasher(&"key2"), |&x| x == "key2", hasher)
        .insert("updated_value");

    // Assert that the value has been updated
    assert_eq!(entry.get(), &"updated_value");
}

