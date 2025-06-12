// Answer 0

#[test]
fn test_get_mut() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use hashbrown::hash_table::Entry;
    use std::hash::BuildHasher;

    // Initialize the hash table
    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert a unique entry
    table.insert_unique(hasher(&"key1"), ("key1", 10), |(k, _)| hasher(&k));

    // Ensure the entry was inserted
    assert_eq!(
        table.find(hasher(&"key1"), |&(x, _)| x == "key1",),
        Some(&("key1", 10))
    );

    // Access the entry and mutate it
    if let Entry::Occupied(mut o) = table.entry(
        hasher(&"key1"),
        |&(x, _)| x == "key1",
        |(k, _)| hasher(&k),
    ) {
        let value_mut = o.get_mut();
        *value_mut = ("key1", value_mut.1 + 5);

        // Check if the mutation is reflected
        assert_eq!(o.get().1, 15);
    }

    assert_eq!(
        table.find(hasher(&"key1"), |&(x, _)| x == "key1",),
        Some(&("key1", 15))
    );

    // Inserting another entry to check multiple mutations
    table.insert_unique(hasher(&"key2"), ("key2", 20), |(k, _)| hasher(&k));
    
    if let Entry::Occupied(mut o) = table.entry(
        hasher(&"key2"),
        |&(x, _)| x == "key2",
        |(k, _)| hasher(&k),
    ) {
        let value_mut = o.get_mut();
        *value_mut = ("key2", value_mut.1 + 10);

        // Check the second mutation
        assert_eq!(o.get().1, 30);
    }

    assert_eq!(
        table.find(hasher(&"key2"), |&(x, _)| x == "key2",),
        Some(&("key2", 30))
    );
}

