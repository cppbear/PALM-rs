// Answer 0

#[test]
fn test_append_with_new_key() {
    let mut map = HeaderMap::with_capacity(10);
    let key: HeaderName = "Test-Key".parse().unwrap();
    let value: HeaderValue = "Test-Value".parse().unwrap();
    map.append(key.clone(), value);
}

#[test]
fn test_append_existing_key() {
    let mut map = HeaderMap::with_capacity(10);
    let key: HeaderName = "Test-Key".parse().unwrap();
    let value1: HeaderValue = "First-Value".parse().unwrap();
    let value2: HeaderValue = "Second-Value".parse().unwrap();
    map.append(key.clone(), value1);
    map.append(key.clone(), value2);
}

#[should_panic(expected = "size overflows MAX_SIZE")]
#[test]
fn test_append_exceeding_max_size() {
    let mut map = HeaderMap::try_with_capacity(32768).unwrap();
    let key: HeaderName = "Overflow-Key".parse().unwrap();
    for i in 0..32768 {
        let value: HeaderValue = format!("Value-{}", i).parse().unwrap();
        map.append(key.clone(), value);
    }
}

#[test]
fn test_append_multiple_keys() {
    let mut map = HeaderMap::with_capacity(5);
    let key1: HeaderName = "Key1".parse().unwrap();
    let key2: HeaderName = "Key2".parse().unwrap();
    let value1: HeaderValue = "Value1".parse().unwrap();
    let value2: HeaderValue = "Value2".parse().unwrap();
    map.append(key1.clone(), value1);
    map.append(key2.clone(), value2);
}

#[test]
fn test_append_non_ascii_key() {
    let mut map = HeaderMap::with_capacity(5);
    let key: HeaderName = "非ASCII".parse().unwrap(); // Non-ASCII key
    let value: HeaderValue = "Value".parse().unwrap();
    map.append(key, value);
}

#[test]
fn test_append_empty_key_value() {
    let mut map = HeaderMap::with_capacity(5);
    let key: HeaderName = "".parse().unwrap(); // Empty key
    let value: HeaderValue = "".parse().unwrap(); // Empty value
    map.append(key, value);
}

#[test]
fn test_append_edge_values() {
    let mut map = HeaderMap::with_capacity(2);
    let key: HeaderName = "Edge-Key".parse().unwrap();
    let value: HeaderValue = "A".repeat(100).parse().unwrap(); // Max length value
    map.append(key.clone(), value);
}

