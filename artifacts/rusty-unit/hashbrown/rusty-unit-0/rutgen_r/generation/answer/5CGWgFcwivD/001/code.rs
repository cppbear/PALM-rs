// Answer 0

#[test]
fn test_insert_into_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create a new hash table and initialize the hasher
    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Attempt to insert a value into a vacant entry
    let entry = table
        .entry(hasher(&"empty"), |&x| x == "empty", hasher)
        .insert("horseyland");

    // Check that the value was inserted correctly
    assert_eq!(entry.get(), &"horseyland");
}

#[test]
#[should_panic]
fn test_insert_into_vacant_entry_with_replacement() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create a new hash table and initialize the hasher
    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert a value, making the entry occupied
    table
        .entry(hasher(&"occupied"), |&x| x == "occupied", hasher)
        .insert("first_value");

    // This insertion should panic since we are trying to insert again into an occupied entry
    let entry = table
        .entry(hasher(&"occupied"), |&x| x == "occupied", hasher)
        .insert("second_value");

    // Check that the old value was replaced if it were to succeed (which it shouldn't)
    assert_eq!(entry.get(), &"second_value");
}

