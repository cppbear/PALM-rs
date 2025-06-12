// Answer 0

#[test]
fn test_shift_remove_index_with_valid_index() {
    // Prepare the necessary structs and types
    let mut indices = hash_table::HashTable::new();
    
    // Create a list of entries
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    // Perform the shift_remove_index operation
    let result = ref_mut.shift_remove_index(1);
    
    // Assert the expected result
    assert_eq!(result, Some((2, 20))); // Should return the key and value at index 1
    assert_eq!(entries.len(), 2); // Entry at index 1 has been removed
    assert_eq!(entries[0], Bucket { hash: HashValue(1), key: 1, value: 10 }); // Entry at index 0 remains
    assert_eq!(entries[1], Bucket { hash: HashValue(3), key: 3, value: 30 }); // Entry at index 1 should now be moved here
}

#[test]
fn test_shift_remove_index_with_invalid_index() {
    // Prepare the necessary structs and types
    let mut indices = hash_table::HashTable::new();
    
    // Create a list of entries
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    // Try to perform the shift_remove_index operation with an invalid index
    let result = ref_mut.shift_remove_index(2); // Index out of bounds
    
    // Assert the expected result
    assert_eq!(result, None); // Should return None since the index is invalid
}

