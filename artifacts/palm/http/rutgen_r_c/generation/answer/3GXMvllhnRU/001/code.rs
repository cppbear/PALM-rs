// Answer 0

#[test]
fn test_values_non_empty() {
    let mut map = HeaderMap::with_capacity(10);
    map.insert(HeaderName::from_static("Host"), HeaderValue::from_static("example.com"));
    map.insert(HeaderName::from_static("Content-Length"), HeaderValue::from_static("123"));

    let values = map.values();
    let mut collected: Vec<&HeaderValue> = values.inner.collect();

    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0], &HeaderValue::from_static("example.com"));
    assert_eq!(collected[1], &HeaderValue::from_static("123"));
}

#[test]
fn test_values_empty() {
    let map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(0);
    let values = map.values();
    let collected: Vec<&HeaderValue> = values.inner.collect();

    assert!(collected.is_empty());
}

#[test]
fn test_values_multiple_appends() {
    let mut map = HeaderMap::with_capacity(10);
    map.append(HeaderName::from_static("Set-Cookie"), HeaderValue::from_static("cookie1=value1"));
    map.append(HeaderName::from_static("Set-Cookie"), HeaderValue::from_static("cookie2=value2"));
    
    let values = map.values();
    let collected: Vec<&HeaderValue> = values.inner.collect();

    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0], &HeaderValue::from_static("cookie1=value1"));
    assert_eq!(collected[1], &HeaderValue::from_static("cookie2=value2"));
}

#[test]
fn test_values_large_map() {
    let mut map = HeaderMap::with_capacity(100);
    for i in 0..100 {
        let key = format!("Key{}", i);
        let value = format!("Value{}", i);
        map.insert(HeaderName::from_static(&key), HeaderValue::from_static(&value));
    }

    let values = map.values();
    let collected: Vec<&HeaderValue> = values.inner.collect();

    assert_eq!(collected.len(), 100);
    for i in 0..100 {
        assert_eq!(collected[i], &HeaderValue::from_static(&format!("Value{}", i)));
    }
}

