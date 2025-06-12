// Answer 0

#[test]
fn test_swap_remove_index_valid() {
    struct TestEntry<K, V> {
        key: K,
        value: V,
    }

    let mut index_map: IndexMapCore<usize, TestEntry<usize, usize>> = IndexMapCore::new();
    
    index_map.entries.push(TestEntry { key: 1, value: 100 });
    index_map.entries.push(TestEntry { key: 2, value: 200 });
    
    let removed_entry = index_map.swap_remove_index(0);
    
    assert_eq!(removed_entry, Some((1, TestEntry { key: 1, value: 100 })));
    assert_eq!(index_map.entries.len(), 1);
    assert_eq!(index_map.entries[0].key, 2);
}

#[test]
fn test_swap_remove_index_empty() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    let removed_entry = index_map.swap_remove_index(0);
    
    assert_eq!(removed_entry, None);
}

#[test]
fn test_swap_remove_index_out_of_bounds() {
    struct TestEntry<K, V> {
        key: K,
        value: V,
    }

    let mut index_map: IndexMapCore<usize, TestEntry<usize, usize>> = IndexMapCore::new();
    
    index_map.entries.push(TestEntry { key: 1, value: 100 });

    // Attempt to remove an entry at an out of bounds index
    let removed_entry = index_map.swap_remove_index(1);
    
    assert_eq!(removed_entry, None);
}

