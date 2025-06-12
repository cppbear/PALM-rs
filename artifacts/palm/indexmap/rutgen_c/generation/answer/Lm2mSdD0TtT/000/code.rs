// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let entries = vec![
        Bucket { key: 0, value: "zero".to_string() },
        Bucket { key: 1, value: "one".to_string() },
        Bucket { key: 2, value: "two".to_string() },
    ];
    map.entries.extend(entries);

    // Test removing an entry at a valid index
    let removed = map.shift_remove_index(1);
    assert_eq!(removed, Some((1, "one".to_string())));
    assert_eq!(map.entries.len(), 2);
    assert_eq!(map.entries[0].value, "zero");
    assert_eq!(map.entries[1].value, "two");
}

#[test]
fn test_shift_remove_index_out_of_bounds() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let entries = vec![
        Bucket { key: 0, value: "zero".to_string() },
        Bucket { key: 1, value: "one".to_string() },
        Bucket { key: 2, value: "two".to_string() },
    ];
    map.entries.extend(entries);

    // Test removing an entry at an out-of-bounds index
    let removed = map.shift_remove_index(3);
    assert_eq!(removed, None);
    assert_eq!(map.entries.len(), 3); // No change in length
}

#[test]
fn test_shift_remove_index_empty() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();

    // Test removing an entry from an empty index map
    let removed = map.shift_remove_index(0);
    assert_eq!(removed, None);
    assert_eq!(map.entries.len(), 0); // Still empty
}

