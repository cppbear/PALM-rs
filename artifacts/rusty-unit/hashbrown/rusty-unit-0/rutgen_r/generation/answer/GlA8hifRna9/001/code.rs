// Answer 0

#[test]
fn test_find_entry_success() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct TestEntry {
        key: i32,
        value: &'static str,
    }

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert a unique entry into the table
    table.insert_unique(hasher(&1), TestEntry { key: 1, value: "a" }, |val| hasher(&val.key));

    // Attempt to find the entry
    if let Ok(entry) = table.find_entry(hasher(&1), |val| val.key == 1) {
        // Verify the found entry
        assert_eq!(entry.bucket.value.key, 1);
        assert_eq!(entry.bucket.value.value, "a");
    } else {
        panic!("Expected to find the entry but did not.");
    }
}

#[test]
fn test_find_entry_not_found() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct TestEntry {
        key: i32,
        value: &'static str,
    }

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert a unique entry into the table
    table.insert_unique(hasher(&1), TestEntry { key: 1, value: "a" }, |val| hasher(&val.key));

    // Attempt to find a non-existent entry
    let result = table.find_entry(hasher(&2), |val| val.key == 2);
    assert!(result.is_err(), "Expected an error when looking for a non-existent entry.");
}

