// Answer 0

#[test]
fn test_header_map_from_iter_empty() {
    let map: HeaderMap<HeaderValue> = HeaderMap::from_iter(Vec::<(HeaderName, HeaderValue)>::new());
    assert!(map.indices.is_empty());
    assert!(map.entries.is_empty());
}

#[test]
fn test_header_map_from_iter_single_entry() {
    let key = HeaderName::try_from("Test-Key").unwrap();
    let value = HeaderValue::from_static("Test-Value");
    let map: HeaderMap<HeaderValue> = HeaderMap::from_iter(vec![(key.clone(), value.clone())]);
    
    assert_eq!(map.indices.len(), 1);
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].key, key);
    assert_eq!(map.entries[0].value, value);
}

#[test]
fn test_header_map_from_iter_multiple_entries() {
    let entries = vec![
        (HeaderName::try_from("Key1").unwrap(), HeaderValue::from_static("Value1")),
        (HeaderName::try_from("Key2").unwrap(), HeaderValue::from_static("Value2")),
        (HeaderName::try_from("Key3").unwrap(), HeaderValue::from_static("Value3")),
    ];
    let map: HeaderMap<HeaderValue> = HeaderMap::from_iter(entries.iter().cloned());
    
    assert_eq!(map.indices.len(), 3);
    assert_eq!(map.entries.len(), 3);
    assert_eq!(map.entries[0].key, HeaderName::try_from("Key1").unwrap());
    assert_eq!(map.entries[1].key, HeaderName::try_from("Key2").unwrap());
    assert_eq!(map.entries[2].key, HeaderName::try_from("Key3").unwrap());
}

#[test]
#[should_panic]
fn test_header_map_from_iter_invalid_key() {
    let entries = vec![
        (HeaderName::try_from("Invalid-Name-#").unwrap_err(), HeaderValue::from_static("Value")),
    ];
    let _map: HeaderMap<HeaderValue> = HeaderMap::from_iter(entries);
}

