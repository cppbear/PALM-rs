// Answer 0

#[test]
fn test_insert_unique() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create a new HashTable.
    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert a unique value with its hash.
    let entry = table.insert_unique(hasher(&1), 1, hasher);

    // Assert that the entry has the expected hash and value.
    assert_eq!(entry.hash, hasher(&1));
    assert_eq!(entry.bucket.value(), &1); // Assuming `bucket` provides a value method for demonstration.
}

#[test]
fn test_insert_multiple_unique() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create a new HashTable.
    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert multiple unique values.
    let entry1 = table.insert_unique(hasher(&1), 1, hasher);
    let entry2 = table.insert_unique(hasher(&2), 2, hasher);

    // Assert that the entries have the expected hashes and values.
    assert_eq!(entry1.hash, hasher(&1));
    assert_eq!(entry1.bucket.value(), &1);
    assert_eq!(entry2.hash, hasher(&2));
    assert_eq!(entry2.bucket.value(), &2);
}

#[test]
#[should_panic]
fn test_insert_duplicate_value() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create a new HashTable.
    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert a value.
    table.insert_unique(hasher(&1), 1, hasher);

    // Attempt to insert a duplicate value.
    // This should panic if the insert_unique function does not allow duplicates.
    table.insert_unique(hasher(&1), 1, hasher);
}

