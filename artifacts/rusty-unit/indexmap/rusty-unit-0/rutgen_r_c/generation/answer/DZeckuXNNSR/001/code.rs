// Answer 0

#[test]
fn test_update_index_existing_entry() {
    let mut table: Indices = hash_table::HashTable::with_capacity(10);
    let hash = HashValue(1);
    table.insert(hash.get(), 42); // Insert an entry to ensure it exists
    update_index(&mut table, hash, 42, 100); // Update the existing entry
    let updated_value = table.get(&hash.get()).unwrap();
    assert_eq!(*updated_value, 100);
}

#[test]
#[should_panic(expected = "index not found")]
fn test_update_index_non_existing_entry() {
    let mut table: Indices = hash_table::HashTable::with_capacity(10);
    let hash = HashValue(2);
    update_index(&mut table, hash, 42, 100); // Attempt to update a non-existing entry
}

#[test]
fn test_update_index_empty_table() {
    let mut table: Indices = hash_table::HashTable::with_capacity(10);
    let hash = HashValue(3);
    update_index(&mut table, hash, 0, 100); // Attempt to update in an empty table
    // Ensure the table remains empty
    assert!(table.is_empty());
}

#[test]
fn test_update_index_multiple_consecutive_updates() {
    let mut table: Indices = hash_table::HashTable::with_capacity(10);
    let hash = HashValue(4);
    table.insert(hash.get(), 10); // Initial entry
    update_index(&mut table, hash, 10, 20); // First update
    update_index(&mut table, hash, 20, 30); // Second update
    let final_value = table.get(&hash.get()).unwrap();
    assert_eq!(*final_value, 30);
}

