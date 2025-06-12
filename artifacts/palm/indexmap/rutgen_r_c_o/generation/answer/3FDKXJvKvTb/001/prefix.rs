// Answer 0

#[test]
fn test_swap_indices_within_bounds() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    let entry1 = IndexedEntry::new(&mut map, 0);
    let entry2 = IndexedEntry::new(&mut map, 1);
    entry1.map.insert_unique(HashValue::from(1), "key1", "value1");
    entry2.map.insert_unique(HashValue::from(2), "key2", "value2");

    entry1.swap_indices(1);
}

#[test]
fn test_swap_indices_with_different_indices() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    let entry1 = IndexedEntry::new(&mut map, 0);
    let entry2 = IndexedEntry::new(&mut map, 2);
    entry1.map.insert_unique(HashValue::from(1), "key1", "value1");
    entry2.map.insert_unique(HashValue::from(3), "key3", "value3");

    entry1.swap_indices(2);
}

#[should_panic]
fn test_swap_indices_with_out_of_bounds_other() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    let entry1 = IndexedEntry::new(&mut map, 0);
    entry1.map.insert_unique(HashValue::from(1), "key1", "value1");

    entry1.swap_indices(1); // This should panic as other is out of bounds
}

#[should_panic]
fn test_swap_indices_with_same_index() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    let entry1 = IndexedEntry::new(&mut map, 0);
    entry1.map.insert_unique(HashValue::from(1), "key1", "value1");

    entry1.swap_indices(0); // This should panic as both indices are the same
}

#[test]
fn test_swap_indices_with_multiple_entries() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    let entry1 = IndexedEntry::new(&mut map, 0);
    let entry2 = IndexedEntry::new(&mut map, 1);
    let entry3 = IndexedEntry::new(&mut map, 2);
    entry1.map.insert_unique(HashValue::from(1), "key1", "value1");
    entry2.map.insert_unique(HashValue::from(2), "key2", "value2");
    entry3.map.insert_unique(HashValue::from(3), "key3", "value3");

    entry1.swap_indices(2); // Swaps entry at index 0 with entry at index 2
}

