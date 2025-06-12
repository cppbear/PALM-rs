// Answer 0

#[test]
fn test_update_index_success() {
    // Define a simple HashTable and populate it
    let mut table: Indices = hash_table::HashTable::with_capacity_and_hasher(10, std::hash::BuildHasher::default());
    let hash_value = HashValue(1);
    
    // Insert an old index into the table
    table.insert(hash_value.get(), 42);
    
    // Now update the index from 42 to 100 using the update_index function
    update_index(&mut table, hash_value, 42, 100);
    
    // Verify that the value at that hash now reflects the new value
    assert_eq!(table.find(hash_value.get()).unwrap(), &100);
}

#[test]
#[should_panic(expected = "index not found")]
fn test_update_index_non_existent_old_value() {
    // Define a HashTable without any entries
    let mut table: Indices = hash_table::HashTable::with_capacity_and_hasher(10, std::hash::BuildHasher::default());
    let hash_value = HashValue(1);
    
    // Attempt to update a non-existent old value
    update_index(&mut table, hash_value, 42, 100);
}

#[test]
fn test_update_index_with_different_values() {
    // Define a simple HashTable and populate it
    let mut table: Indices = hash_table::HashTable::with_capacity_and_hasher(10, std::hash::BuildHasher::default());
    let hash_value = HashValue(2);

    // Insert the old index into the table
    table.insert(hash_value.get(), 30);
    
    // Update index from 30 to 75
    update_index(&mut table, hash_value, 30, 75);
    
    // Verify the updated value is now 75
    assert_eq!(table.find(hash_value.get()).unwrap(), &75);
}

