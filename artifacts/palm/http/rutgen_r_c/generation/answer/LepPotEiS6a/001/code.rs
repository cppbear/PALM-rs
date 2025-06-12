// Answer 0

#[test]
fn test_keys_len_empty_map() {
    let map: HeaderMap = HeaderMap::with_capacity(10);
    assert_eq!(0, map.keys_len());
}

#[test]
fn test_keys_len_two_unique_keys() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("Key1", "Value1");
    map.insert("Key2", "Value2");
    assert_eq!(2, map.keys_len());
}

#[test]
fn test_keys_len_same_key_different_values() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("Key1", "Value1");
    map.insert("Key1", "Value2");
    assert_eq!(1, map.keys_len());
}

#[test]
fn test_keys_len_multiple_entries_same_key() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("Key1", "Value1");
    map.insert("Key1", "Value2");
    map.insert("Key1", "Value3");
    assert_eq!(1, map.keys_len());
}

#[test]
fn test_keys_len_after_clear() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("Key1", "Value1");
    map.insert("Key2", "Value2");
    map.clear();
    assert_eq!(0, map.keys_len());
}

