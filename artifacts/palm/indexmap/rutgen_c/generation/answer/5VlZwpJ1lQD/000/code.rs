// Answer 0

#[test]
fn test_len_empty() {
    let map: IndexMapCore<usize, String> = IndexMapCore::new();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_len_after_insert() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let key = 1;
    let value = String::from("value1");
    map.push_entry(HashValue::default(), key, value);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_len_after_clear() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let key = 1;
    let value = String::from("value1");
    map.push_entry(HashValue::default(), key, value);
    map.clear();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_len_after_truncate() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let key1 = 1;
    let value1 = String::from("value1");
    let key2 = 2;
    let value2 = String::from("value2");
    map.push_entry(HashValue::default(), key1, value1);
    map.push_entry(HashValue::default(), key2, value2);
    map.truncate(1);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_len_after_split_off() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    for i in 0..5 {
        map.push_entry(HashValue::default(), i, format!("value{}", i));
    }
    let split_map = map.split_off(3);
    assert_eq!(map.len(), 3);
    assert_eq!(split_map.len(), 2);
}

