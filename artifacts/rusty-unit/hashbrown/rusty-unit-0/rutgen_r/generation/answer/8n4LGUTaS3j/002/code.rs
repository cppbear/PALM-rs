// Answer 0

#[test]
fn test_and_modify_with_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    // Insert an initial value to ensure the entry is occupied
    table.insert(("example", 10));
    
    let entry = table.entry(
        hasher(&"example"),
        |&(x, _)| x == "example",
        |(k, _)| hasher(&k),
    );
    
    // Call and_modify on the occupied entry
    let modified_entry = entry.and_modify(|(_, v)| *v += 5);
    assert!(matches!(modified_entry, Entry::Occupied(_)));

    let result = table.find(hasher(&"example"), |&(k, _)| k == "example");
    assert_eq!(result, Some(&("example", 15)));
}

#[test]
#[should_panic]
fn test_and_modify_with_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Attempt to modify a non-existing entry should panic
    let entry = table.entry(
        hasher(&"nonexistent"),
        |&(x, _)| x == "nonexistent",
        |(k, _)| hasher(&k),
    );

    // This will trigger the panic since it is a vacant entry
    entry.and_modify(|(_, _)| panic!("This should not be called"));
}

