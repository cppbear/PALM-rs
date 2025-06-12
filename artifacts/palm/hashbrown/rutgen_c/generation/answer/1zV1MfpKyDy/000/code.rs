// Answer 0

#[cfg(test)]
fn test_hash_table_iter() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Initialize a HashTable instance
    let mut table: HashTable<&str> = HashTable::new_in(DefaultHashBuilder::default());

    // Define a simple hasher function
    let hasher = |val: &_| DefaultHashBuilder::default().hash_one(val);

    // Insert some unique elements
    table.insert_unique(hasher(&"a"), "b", hasher);
    table.insert_unique(hasher(&"b"), "b", hasher);

    // Use the iterator and collect the results
    let collected: Vec<&str> = table.iter().collect();

    // Check if the collector contains the expected items
    assert!(collected.contains(&"b"), "Should contain 'b'");
    assert_eq!(collected.len(), 1, "Should contain exactly one unique item");
}

#[cfg(test)]
fn test_hash_table_iter_empty() {
    use hashbrown::HashTable;

    // Initialize an empty HashTable instance
    let table: HashTable<i32> = HashTable::new_in(DefaultHashBuilder::default());

    // Use the iterator on an empty table
    let collected: Vec<i32> = table.iter().collect();

    // Check that it is empty
    assert!(collected.is_empty(), "Iterator should be empty for an empty table");
}

