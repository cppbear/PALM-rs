// Answer 0

#[test]
fn test_and_modify_on_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert an entry into the table to ensure it exists
    table.insert_unique(hasher(&"poneyland"), ("poneyland", 40), hasher);

    // Modify the entry by incrementing the value
    let entry = table.entry(
        hasher(&"poneyland"),
        |&(x, _)| x == "poneyland",
        |(k, _)| hasher(&k),
    );

    let modified_entry = entry.and_modify(|(_, v)| *v += 2);

    match modified_entry {
        Entry::Occupied(_) => {
            // Verify the value has been modified correctly
            assert_eq!(*table.find(hasher(&"poneyland"), |&(k, _)| k == "poneyland").unwrap(), 42);
        },
        Entry::Vacant(_) => panic!("Expected an occupied entry, but found vacant"),
    }
}

#[test]
fn test_and_modify_on_non_existent_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Attempt to modify a non-existent entry
    let entry = table.entry(
        hasher(&"non_existent"),
        |&(x, _)| x == "non_existent",
        |(k, _)| hasher(&k),
    );

    // This should remain vacant
    let modified_entry = entry.and_modify(|(_, _)| panic!("Should not modify"));

    match modified_entry {
        Entry::Occupied(_) => panic!("Expected a vacant entry, but found occupied"),
        Entry::Vacant(_) => {}
    }
}

