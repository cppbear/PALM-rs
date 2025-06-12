// Answer 0

#[test]
fn test_contains_key_empty_map() {
    let map: HeaderMap = HeaderMap::with_capacity(10);
    let key = "non_existing_key";
    map.contains_key(key);
}

#[test]
fn test_contains_key_after_insert() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    let key = "X-Test-Header";
    map.insert(key, "value".parse().unwrap());
    map.contains_key(key);
}

#[test]
fn test_contains_key_case_insensitivity() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    let key = "TEST-HEADER";
    map.insert(key, "value".parse().unwrap());
    map.contains_key("test-header");
}

#[test]
fn test_contains_key_with_different_key() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    let key1 = "X-Header-1";
    let key2 = "X-Header-2";
    map.insert(key1, "value1".parse().unwrap());
    map.contains_key(key2);
}

#[test]
fn test_contains_key_numeric_header_name() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    let key = "123-Header";
    map.insert(key, "value".parse().unwrap());
    map.contains_key(key);
}

#[test]
fn test_contains_key_edge_case_large_capacity() {
    let mut map: HeaderMap = HeaderMap::try_with_capacity(65535).unwrap();
    let key = "X-Large-Capacity-Header";
    map.insert(key, "large_value".parse().unwrap());
    map.contains_key(key);
}

#[test]
fn test_contains_key_with_special_characters() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    let key = "X-Header-With-$pecial*Chars";
    map.insert(key, "value".parse().unwrap());
    map.contains_key(key);
}

