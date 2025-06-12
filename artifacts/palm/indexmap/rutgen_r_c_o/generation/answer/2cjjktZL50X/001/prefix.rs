// Answer 0

#[test]
fn test_indexed_entry_new_valid() {
    let mut map = IndexMapCore::new();
    let index = 0;
    let entry = IndexedEntry::new(&mut map, index);
}

#[test]
fn test_indexed_entry_new_max_index() {
    let mut map = IndexMapCore::with_capacity(10);
    let index = IndexMapCore::MAX_ENTRIES_CAPACITY;
    let entry = IndexedEntry::new(&mut map, index);
}

#[test]
#[should_panic] // Expecting panic due to out-of-bounds index
fn test_indexed_entry_new_out_of_bounds() {
    let mut map = IndexMapCore::new();
    let index = 1; // Assuming this index exceeds the initial length which is 0
    let _entry = IndexedEntry::new(&mut map, index);
}

#[test]
fn test_indexed_entry_new_non_zero_index() {
    let mut map = IndexMapCore::with_capacity(3);
    let _ = map.entries.push(("key1", "value1"));
    let index = 0; // Valid index within the map
    let entry = IndexedEntry::new(&mut map, index);
}

#[test]
fn test_indexed_entry_new_empty_map() {
    let mut map = IndexMapCore::new();
    let index = 0; // Valid index, but map is empty
    let entry = IndexedEntry::new(&mut map, index);
}

