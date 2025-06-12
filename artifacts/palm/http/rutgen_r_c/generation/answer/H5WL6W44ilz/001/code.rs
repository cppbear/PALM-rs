// Answer 0

#[test]
fn test_insert_empty_map() {
    let mut map = HeaderMap::with_capacity(1);
    assert!(map.insert("Test-Header", "value1").is_none());
    assert_eq!(map.len(), 1);
}

#[test]
fn test_insert_existing_key() {
    let mut map = HeaderMap::with_capacity(1);
    map.insert("Test-Header", "value1");
    let prev = map.insert("Test-Header", "value2").unwrap();
    assert_eq!(prev, "value1");
    assert_eq!(map.len(), 1);
}

#[test]
fn test_insert_multiple_values() {
    let mut map = HeaderMap::with_capacity(2);
    map.insert("Test-Header", "value1");
    map.insert("Test-Header", "value2");
    map.insert("Test-Header", "value3");
    let prev = map.insert("Test-Header", "value4").unwrap();
    assert_eq!(prev, "value3");
    assert_eq!(map.len(), 1);
}

#[test]
#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_insert_over_max_size() {
    let mut map = HeaderMap::try_with_capacity(MAX_SIZE + 1).unwrap_err();
    map.insert("Test-Header", "value1").unwrap();
}

#[test]
fn test_insert_empty_key() {
    let mut map = HeaderMap::with_capacity(1);
    assert!(map.insert("", "value1").is_none());
    assert_eq!(map.len(), 1);
}

#[test]
fn test_insert_same_key_different_values() {
    let mut map = HeaderMap::with_capacity(2);
    map.insert("Header-A", "value1");
    assert_eq!(map.insert("Header-A", "value2").unwrap(), "value1");
    assert_eq!(map.insert("Header-A", "value3").unwrap(), "value2");
}

#[test]
fn test_insert_with_capacity_edge_case() {
    let mut map = HeaderMap::with_capacity(1);
    map.insert("Single-Header", "first_value");
    let mut result = map.insert("Single-Header", "new_value").unwrap();
    assert_eq!(result, "first_value");
    assert_eq!(map.len(), 1);
}

