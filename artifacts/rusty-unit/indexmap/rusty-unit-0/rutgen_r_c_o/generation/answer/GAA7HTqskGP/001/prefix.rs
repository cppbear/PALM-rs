// Answer 0

#[test]
fn test_get_mut_valid_index() {
    let mut map = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: "key1", value: 42 });
    let mut entry = IndexedEntry::new(&mut map, 0);
    entry.get_mut();
}

#[test]
#[should_panic]
fn test_get_mut_invalid_index_high() {
    let mut map = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: "key1", value: 42 });
    let mut entry = IndexedEntry::new(&mut map, 1);
    entry.get_mut();
}

#[test]
#[should_panic]
fn test_get_mut_invalid_index_low() {
    let mut map = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: "key1", value: 42 });
    let mut entry = IndexedEntry::new(&mut map, usize::MAX);
    entry.get_mut();
}

#[test]
fn test_get_mut_multiple_entries() {
    let mut map = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: "key1", value: 42 });
    map.entries.push(Bucket { hash: HashValue::default(), key: "key2", value: 84 });
    let mut entry = IndexedEntry::new(&mut map, 1);
    entry.get_mut();
}

#[test]
fn test_get_mut_empty_map() {
    let mut map = IndexMapCore::new();
    let mut entry = IndexedEntry::new(&mut map, 0);
    entry.get_mut(); // Should panic due to empty entries
}

