// Answer 0

#[test]
fn test_get_mut() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create a mutable HashTable
    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert an initial entry
    table.insert_unique(hasher(&"poneyland"), ("poneyland", 12), |(k, _)| hasher(&k));

    // Check if the entry exists as expected
    assert_eq!(
        table.find(hasher(&"poneyland"), |&(x, _)| x == "poneyland",),
        Some(&("poneyland", 12))
    );

    // Check for the entry and get a mutable reference
    if let Entry::Occupied(mut o) = table.entry(
        hasher(&"poneyland"),
        |&(x, _)| x == "poneyland",
        |(k, _)| hasher(&k),
    ) {
        o.get_mut().1 += 10;  // Increment the value
        assert_eq!(o.get().1, 22); // Verify the increment

        // Increment the value again
        o.get_mut().1 += 2; 
    }

    // Verify the final value
    assert_eq!(
        table.find(hasher(&"poneyland"), |&(x, _)| x == "poneyland",),
        Some(&("poneyland", 24))
    );
}

