// Answer 0

#[test]
fn test_insert_occupied_entry_replaces_value() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &&str| hasher.hash_one(val);

    // Insert an initial value.
    let _ = table
        .entry(hasher(&"key"), |&x| x == "key", hasher)
        .insert("initial_value");

    // Retrieve the occupied entry and replace its value.
    let entry = table
        .entry(hasher(&"key"), |&x| x == "key", hasher)
        .insert("new_value");

    assert_eq!(entry.get(), &"new_value");
}

#[test]
#[should_panic(expected = "panic expected if trying to access a vacant entry")]
fn test_insert_vacant_entry_panics() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &&str| hasher.hash_one(val);

    // Attempt to insert a new value without having a corresponding occupied entry.
    let _ = table
        .entry(hasher(&"nonexistent_key"), |&x| x == "nonexistent_key", hasher)
        .insert("some_value"); // This should not panic as it's being inserted but won't replace.

    // Now we will expect a panic if we try to access it as occupied without inserting first.
    let _: &str = table
        .entry(hasher(&"nonexistent_key"), |&x| x == "nonexistent_key", hasher)
        .insert("new_value")
        .get(); // This will panic since the entry is still vacant.
}

