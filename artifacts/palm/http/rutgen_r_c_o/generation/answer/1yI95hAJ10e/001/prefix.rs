// Answer 0

#[test]
fn test_empty_header_map() {
    let map: HeaderMap = HeaderMap::with_capacity(0);
    map.is_empty();
}

#[test]
fn test_non_empty_header_map() {
    let mut map: HeaderMap = HeaderMap::with_capacity(1);
    let key = HeaderName::from_str("Host").unwrap();
    map.insert(key, HeaderValue::from_str("hello.world").unwrap());
    map.is_empty();
}

#[test]
fn test_empty_after_clear() {
    let mut map: HeaderMap = HeaderMap::with_capacity(1);
    let key = HeaderName::from_str("Host").unwrap();
    map.insert(key, HeaderValue::from_str("hello.world").unwrap());
    map.clear();
    map.is_empty();
}

#[test]
fn test_multiple_entries() {
    let mut map: HeaderMap = HeaderMap::with_capacity(2);
    let key1 = HeaderName::from_str("Host").unwrap();
    let key2 = HeaderName::from_str("User-Agent").unwrap();
    map.insert(key1, HeaderValue::from_str("hello.world").unwrap());
    map.insert(key2, HeaderValue::from_str("my-agent").unwrap());
    map.is_empty();
}

#[test]
#[should_panic]
fn test_out_of_capacity() {
    let mut map: HeaderMap = HeaderMap::try_with_capacity(1).unwrap();
    let key = HeaderName::from_str("Host").unwrap();
    let _ = map.try_insert(key, HeaderValue::from_str("hello.world").unwrap()).unwrap();
    let _ = map.try_insert(key, HeaderValue::from_str("new.value").unwrap()).unwrap();
}

