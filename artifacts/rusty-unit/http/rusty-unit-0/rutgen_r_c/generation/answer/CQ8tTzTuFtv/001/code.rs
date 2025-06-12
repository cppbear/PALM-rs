// Answer 0

#[test]
fn test_iter_empty_header_map() {
    let map: HeaderMap = HeaderMap::with_capacity(0);
    let iterator = map.iter();
    assert_eq!(iterator.entry, 0);
    assert!(iterator.cursor.is_none());
}

#[test]
fn test_iter_single_entry_header_map() {
    let mut map = HeaderMap::with_capacity(1);
    let key = HeaderName::from_static("Key1");
    let value = HeaderValue::from_static("Value1");
    map.insert(key.clone(), value.clone());

    let iterator = map.iter();
    assert_eq!(iterator.entry, 0);
    assert!(matches!(iterator.cursor, Some(Cursor::Head)));
    
    let (iter_key, iter_value) = map.iter().next().unwrap(); // assuming `next` method is defined
    assert_eq!(iter_key, key);
    assert_eq!(iter_value, value);
}

#[test]
fn test_iter_multiple_entries_header_map() {
    let mut map = HeaderMap::with_capacity(2);
    let key1 = HeaderName::from_static("Key1");
    let value1 = HeaderValue::from_static("Value1");
    let key2 = HeaderName::from_static("Key2");
    let value2 = HeaderValue::from_static("Value2");
    
    map.insert(key1.clone(), value1.clone());
    map.append(key1.clone(), value1.clone()); // Appending to generate multiple values for the same key
    map.insert(key2.clone(), value2.clone());

    let iterator = map.iter();
    assert_eq!(iterator.entry, 0);
    assert!(matches!(iterator.cursor, Some(Cursor::Head)));

    let (iter_key1, iter_value1) = map.iter().next().unwrap(); // assuming `next` method is defined
    assert_eq!(iter_key1, key1);
    assert_eq!(iter_value1, value1);
    
    let (iter_key1_2, iter_value1_2) = map.iter().next().unwrap(); // second value for the same key
    assert_eq!(iter_key1_2, key1);
    assert_eq!(iter_value1_2, value1);
    
    let (iter_key2, iter_value2) = map.iter().next().unwrap(); // next key-value
    assert_eq!(iter_key2, key2);
    assert_eq!(iter_value2, value2);
}

