// Answer 0

#[test]
fn test_and_modify_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let _ = table
        .entry(
            hasher(&"poneyland"),
            |&(x, _)| x == "poneyland",
            |(k, _)| hasher(&k),
        )
        .or_insert(("poneyland", 42));

    table
        .entry(
            hasher(&"poneyland"),
            |&(x, _)| x == "poneyland",
            |(k, _)| hasher(&k),
        )
        .and_modify(|(_, v)| *v += 1);

    assert_eq!(
        table.find(hasher(&"poneyland"), |&(k, _)| k == "poneyland"),
        Some(&("poneyland", 43))
    );
}

#[test]
fn test_and_modify_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let entry = table
        .entry(
            hasher(&"nonexistent"),
            |&(x, _)| x == "nonexistent",
            |(k, _)| hasher(&k),
        );

    let modified_entry = entry.and_modify(|(_, v)| *v += 1);
    
    if let Entry::Vacant(_) = modified_entry {
        // The entry is vacant, so we can assert here.
        assert!(true);
    } else {
        panic!("Expected entry to be vacant");
    }
}

