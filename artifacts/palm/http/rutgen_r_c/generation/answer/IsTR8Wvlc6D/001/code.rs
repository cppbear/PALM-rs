// Answer 0

#[test]
fn test_get_empty_map() {
    let mut map: HeaderMap<&str> = HeaderMap::with_capacity(10);
    assert!(map.get("host").is_none());
}

#[test]
fn test_get_existing_key() {
    let mut map: HeaderMap<&str> = HeaderMap::with_capacity(10);
    map.insert("host", "hello").unwrap();
    assert_eq!(map.get("host").unwrap(), &"hello");
}

#[test]
fn test_get_case_insensitive_key() {
    let mut map: HeaderMap<&str> = HeaderMap::with_capacity(10);
    map.insert("HOST", "hello").unwrap();
    assert_eq!(map.get("host").unwrap(), &"hello");
}

#[test]
fn test_get_multiple_values_for_same_key() {
    let mut map: HeaderMap<&str> = HeaderMap::with_capacity(10);
    map.insert("host", "hello").unwrap();
    map.append("host", "world").unwrap();
    assert_eq!(map.get("host").unwrap(), &"hello");
}

#[test]
fn test_get_non_existent_key() {
    let mut map: HeaderMap<&str> = HeaderMap::with_capacity(10);
    map.insert("host", "hello").unwrap();
    assert!(map.get("non_existent").is_none());
}

#[test]
fn test_get_value_after_inserting_and_appending() {
    let mut map: HeaderMap<&str> = HeaderMap::with_capacity(10);
    map.insert("host", "hello").unwrap();
    map.append("host", "world").unwrap();
    assert_eq!(map.get("host").unwrap(), &"hello");
}

