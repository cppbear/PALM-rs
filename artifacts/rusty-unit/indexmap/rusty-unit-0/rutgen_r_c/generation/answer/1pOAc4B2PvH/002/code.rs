// Answer 0

#[test]
fn test_retain_in_order_with_equal_length() {
    use crate::{HashValue, Equivalent};
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();

    // Initially insert some entries
    let hash_value_1 = HashValue::from(1);
    let hash_value_2 = HashValue::from(2);
    index_map.push_entry(hash_value_1, 1, 10);
    index_map.push_entry(hash_value_2, 2, 20);
    
    // Simulate that entries and indices have the same length
    let expected_len = index_map.entries.len();
    index_map.indices.insert(1, 0); // Simulate that 1 index is present
    index_map.indices.insert(2, 1); // Simulate that another index is present

    // Retain entries where the key is even (should retain only the entry with key 2)
    index_map.retain_in_order(|k, v| *k % 2 == 0);

    // Assert that only the entry with key 2 remains
    assert_eq!(index_map.entries.len(), 1);
    assert_eq!(index_map.entries[0].key, 2);
    assert_eq!(index_map.entries[0].value, 20);

    // Assert that the state of indices remains consistent (should still equal entries)
    assert_eq!(index_map.indices.len(), expected_len);
}

#[test]
fn test_retain_in_order_with_no_entries_kept() {
    use crate::{HashValue, Equivalent};
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();

    // Initially insert some entries
    let hash_value_1 = HashValue::from(1);
    let hash_value_2 = HashValue::from(2);
    index_map.push_entry(hash_value_1, 1, 10);
    index_map.push_entry(hash_value_2, 2, 20);
    
    // Setup indices to match entries
    index_map.indices.insert(1, 0);
    index_map.indices.insert(2, 1);

    // Retain entries where no keys are kept (should result in an empty entries vec)
    index_map.retain_in_order(|_k, _v| false);

    // Assert that no entries remain
    assert_eq!(index_map.entries.len(), 0); 
    assert_eq!(index_map.indices.len(), 2); // Indices remain unchanged
}

#[test]
fn test_retain_in_order_edge_case_with_one_entry() {
    use crate::{HashValue, Equivalent};
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();

    // Insert a single entry
    let hash_value = HashValue::from(1);
    index_map.push_entry(hash_value, 1, 10);

    // Setup indices to match the single entry
    index_map.indices.insert(1, 0);

    // Retain that entry by keeping all entries
    index_map.retain_in_order(|_k, _v| true);

    // Assert that the single entry remains
    assert_eq!(index_map.entries.len(), 1);
    assert_eq!(index_map.entries[0].key, 1);
    assert_eq!(index_map.entries[0].value, 10);

    // Check that indices remain in sync
    assert_eq!(index_map.indices.len(), 1);
}

