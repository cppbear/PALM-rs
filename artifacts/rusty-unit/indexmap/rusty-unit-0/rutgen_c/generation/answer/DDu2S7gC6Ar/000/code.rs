// Answer 0

#[test]
fn test_replace_full_new_entry() {
    let mut map: IndexMapCore<i32, &str> = IndexMapCore::new();
    let hash = HashValue(1);
    let (index, replaced) = map.replace_full(hash, 42, "Test Value");
    assert_eq!(index, 0);
    assert!(replaced.is_none());
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].key, 42);
    assert_eq!(map.entries[0].value, "Test Value");
}

#[test]
fn test_replace_full_existing_entry() {
    let mut map: IndexMapCore<i32, &str> = IndexMapCore::new();
    let hash = HashValue(2);
    let _ = map.replace_full(hash, 42, "Initial Value");

    let (index, replaced) = map.replace_full(hash, 100, "Updated Value");
    assert_eq!(index, 0);
    assert_eq!(replaced, Some((42, "Initial Value")));
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].key, 100);
    assert_eq!(map.entries[0].value, "Updated Value");
}

#[test]
fn test_replace_full_multiple_entries() {
    let mut map: IndexMapCore<i32, &str> = IndexMapCore::new();
    let hash1 = HashValue(3);
    let hash2 = HashValue(4);
    let _ = map.replace_full(hash1, 42, "Value 1");
    let _ = map.replace_full(hash2, 99, "Value 2");

    let (index, replaced) = map.replace_full(hash1, 100, "Updated Value 1");
    assert_eq!(index, 0);
    assert_eq!(replaced, Some((42, "Value 1")));
    
    let (index2, replaced2) = map.replace_full(hash2, 101, "Updated Value 2");
    assert_eq!(index2, 1);
    assert_eq!(replaced2, Some((99, "Value 2")));
    
    assert_eq!(map.entries.len(), 2);
    assert_eq!(map.entries[0].key, 100);
    assert_eq!(map.entries[0].value, "Updated Value 1");
    assert_eq!(map.entries[1].key, 101);
    assert_eq!(map.entries[1].value, "Updated Value 2");
}

