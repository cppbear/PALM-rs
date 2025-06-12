// Answer 0

#[test]
fn test_into_mut_valid_index() {
    // Arrange
    let mut entries = Vec::new();
    entries.push(Bucket { hash: HashValue::default(), key: "key1", value: 10 });
    entries.push(Bucket { hash: HashValue::default(), key: "key2", value: 20 });

    let mut indices = Indices::new();
    let mut map_core = IndexMapCore::new_with_entries(entries.clone(), indices.clone());

    let index = 0; // Valid index
    let indexed_entry = IndexedEntry::new(&mut map_core, index);

    // Act
    let value_mut_ref = indexed_entry.into_mut();

    // Assert
    assert_eq!(*value_mut_ref, 10);
    *value_mut_ref = 30; // Modify the value
    
    assert_eq!(map_core.entries[index].value, 30); // Check if the original map updated
}

#[test]
#[should_panic]
fn test_into_mut_invalid_index() {
    // Arrange
    let mut entries = Vec::new();
    entries.push(Bucket { hash: HashValue::default(), key: "key1", value: 10 });
    let mut indices = Indices::new();
    let mut map_core = IndexMapCore::new_with_entries(entries, indices);

    let index = 1; // Invalid index, out of bounds
    let indexed_entry = IndexedEntry::new(&mut map_core, index);

    // Act
    let _value_mut_ref = indexed_entry.into_mut(); // This should cause a panic
}

#[test]
fn test_into_mut_edge_case() {
    // Arrange
    let mut entries = Vec::new();
    entries.push(Bucket { hash: HashValue::default(), key: "key1", value: 10 });
    entries.push(Bucket { hash: HashValue::default(), key: "key2", value: 20 });

    let mut indices = Indices::new();
    let mut map_core = IndexMapCore::new_with_entries(entries.clone(), indices.clone());

    let index = 1; // Edge case index
    let indexed_entry = IndexedEntry::new(&mut map_core, index);

    // Act
    let value_mut_ref = indexed_entry.into_mut();

    // Assert
    assert_eq!(*value_mut_ref, 20);
    *value_mut_ref = 40; // Modify the value
    
    assert_eq!(map_core.entries[index].value, 40); // Check if the original map updated
}

