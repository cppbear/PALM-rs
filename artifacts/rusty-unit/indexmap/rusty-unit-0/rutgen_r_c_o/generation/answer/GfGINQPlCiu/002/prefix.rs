// Answer 0

#[test]
fn test_get_index_entry_valid_index_zero() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    let entry = map.get_index_entry(0);
}

#[test]
fn test_get_index_entry_valid_index_mid() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    let entry = map.get_index_entry(1);
}

#[test]
fn test_get_index_entry_valid_index_last() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    let entry = map.get_index_entry(1);
}

#[test]
fn test_get_index_entry_return_none_out_of_bounds() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    let entry = map.get_index_entry(1);
}

