// Answer 0

#[test]
fn test_key_mut_valid_index() {
    use crate::{IndexMapCore, HashValue, Entries};
    use std::collections::HashMap;

    // Set up an IndexMapCore with some entries
    let mut index_map = IndexMapCore::new();
    let key1 = "key1";
    let value1 = 42;
    index_map.insert(key1, value1);

    // Create an IndexedEntry at a valid index
    let mut indexed_entry = IndexedEntry::new(&mut index_map, 0);

    // Access the mutable key
    let key_mut = indexed_entry.key_mut();
    
    // Ensure that the mutable key can be modified
    *key_mut = "new_key1";

    // Verify the value has changed in the map
    assert_eq!(index_map.get("new_key1"), Some(&value1));
}

#[test]
#[should_panic(expected = "index out of bounds")]  // Assuming the panic message when index is out of bounds
fn test_key_mut_invalid_index() {
    use crate::{IndexMapCore, HashValue, Entries};
    
    // Set up an IndexMapCore with some entries
    let mut index_map = IndexMapCore::new();
    let key1 = "key1";
    let value1 = 42;
    index_map.insert(key1, value1);

    // Create an IndexedEntry at an invalid index
    let mut indexed_entry = IndexedEntry::new(&mut index_map, 1); // Index 1 does not exist

    // Access the mutable key, which should panic
    let _key_mut = indexed_entry.key_mut();
}

#[test]
fn test_key_mut_multiple_entries() {
    use crate::{IndexMapCore};

    // Set up an IndexMapCore with multiple entries
    let mut index_map = IndexMapCore::new();
    let key1 = "key1";
    let value1 = 42;
    let key2 = "key2";
    let value2 = 100;
    index_map.insert(key1, value1);
    index_map.insert(key2, value2);

    // Create an IndexedEntry for the second entry
    let mut indexed_entry = IndexedEntry::new(&mut index_map, 1);

    // Access the mutable key
    let key_mut = indexed_entry.key_mut();
    
    // Ensure that the mutable key can be modified
    *key_mut = "new_key2";

    // Verify the value has changed in the map
    assert_eq!(index_map.get("new_key2"), Some(&value2));
}

#[test]
#[should_panic(expected = "index out of bounds")] // Assuming that the panic message when index is out of bounds
fn test_key_mut_empty_map() {
    use crate::{IndexMapCore};

    // Set up an empty IndexMapCore
    let mut index_map = IndexMapCore::new();

    // Create an IndexedEntry at an invalid index (0) since the map is empty
    let mut indexed_entry = IndexedEntry::new(&mut index_map, 0); 

    // Access the mutable key, which should panic
    let _key_mut = indexed_entry.key_mut();
}

