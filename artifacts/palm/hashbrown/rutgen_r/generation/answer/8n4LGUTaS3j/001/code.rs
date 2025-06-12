// Answer 0

#[test]
fn test_and_modify_vacant_entry() {
    use hashbrown::{HashTable, Entry, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Create a vacant entry in the table
    let entry = table.entry(
        hasher(&"nonexistent"),
        |&(x, _)| x == "nonexistent",
        |(k, _)| hasher(&k),
    );

    // Now call and_modify on the vacant entry
    let result = entry.and_modify(|(_, v)| *v += 1);

    // Test that we get a vacant entry back
    if let Entry::Vacant(vacant_entry) = result {
        assert!(vacant_entry.key() == hasher(&"nonexistent"));
    } else {
        panic!("Expected Entry::Vacant, but got Entry::Occupied");
    }
}

