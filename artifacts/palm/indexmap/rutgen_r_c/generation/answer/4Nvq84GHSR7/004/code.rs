// Answer 0

#[test]
fn test_erase_indices_degenerate_case() {
    let mut map: IndexMapCore<u32, u32> = IndexMapCore::new();
    map.indices = Indices::with_capacity(10);
    map.entries = vec![Bucket { hash: HashValue(0), key: 1, value: 2 }];

    // Call erase_indices with start = 0 and end = 0, should do nothing.
    map.erase_indices(0, 0);
    assert_eq!(map.indices.len(), 0);
}

#[test]
fn test_erase_indices_small_erased() {
    let mut map: IndexMapCore<u32, u32> = IndexMapCore::with_capacity(10);
    
    map.entries.push(Bucket { hash: HashValue(0), key: 1, value: 2 });
    map.entries.push(Bucket { hash: HashValue(1), key: 2, value: 3 });
    
    map.indices.insert(0, 0);
    map.indices.insert(1, 1);

    // Call erase_indices with start = 0, end = 1.
    map.erase_indices(0, 1);
    assert_eq!(map.indices.len(), 1);
    assert_eq!(map.indices.get(0), Some(&1));
}

#[test]
fn test_erase_indices_start_plus_shifted_less_half_capacity() {
    let mut map: IndexMapCore<u32, u32> = IndexMapCore::with_capacity(10);

    // Initial entries
    map.entries.push(Bucket { hash: HashValue(0), key: 1, value: 2 });
    map.entries.push(Bucket { hash: HashValue(1), key: 2, value: 3 });
    map.entries.push(Bucket { hash: HashValue(2), key: 3, value: 4 });

    // Setup indices
    map.indices.insert(0, 0);
    map.indices.insert(1, 1);
    map.indices.insert(2, 2);

    // Call erase_indices with start = 0, end = 2, should remove index 0 and update index 1 to 0.
    map.erase_indices(0, 2);
    assert_eq!(map.indices.len(), 1);
    assert_eq!(map.indices.get(0), Some(&1)); // Only one index should remain
}

#[test]
fn test_erase_indices_shift_adjustment() {
    let mut map: IndexMapCore<u32, u32> = IndexMapCore::with_capacity(10);

    // Initial entries
    map.entries.push(Bucket { hash: HashValue(0), key: 1, value: 2 });
    map.entries.push(Bucket { hash: HashValue(1), key: 2, value: 3 });
    map.entries.push(Bucket { hash: HashValue(2), key: 3, value: 4 });
    
    // Setup indices
    map.indices.insert(0, 0);
    map.indices.insert(1, 1);
    map.indices.insert(2, 2);

    // Call erase_indices with start = 1, end = 3, should remove indices 1 and 2 and adjust indexed values.
    map.erase_indices(1, 3);
    assert_eq!(map.indices.len(), 1); // Should be one index left
    assert_eq!(map.indices.get(0), Some(&0)); // Index should have shifted down
}

#[should_panic]
#[test]
fn test_erase_indices_panic_due_to_empty_entries() {
    let mut map: IndexMapCore<u32, u32> = IndexMapCore::new();
  
    // This should panic, as the entries are empty and we are trying to split at 0
    map.erase_indices(0, 1);
}

#[should_panic]
#[test]
fn test_erase_indices_panic_due_to_start_out_of_bounds() {
    let mut map: IndexMapCore<u32, u32> = IndexMapCore::new();

    // Populate with one entry for testing
    map.entries.push(Bucket { hash: HashValue(0), key: 1, value: 2 });

    // This should panic since end is greater than the length of entries
    map.erase_indices(1, 2);
}

