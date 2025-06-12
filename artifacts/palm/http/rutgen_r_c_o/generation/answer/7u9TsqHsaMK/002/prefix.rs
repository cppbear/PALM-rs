// Answer 0

#[test]
fn test_drain_with_non_empty_map() {
    let mut map = HeaderMap::with_capacity(10);
    map.insert(HeaderName::from_static("Host"), "example.com".parse().unwrap());
    map.append(HeaderName::from_static("Host"), "sub.example.com".parse().unwrap());
    map.insert(HeaderName::from_static("Content-Length"), "123".parse().unwrap());

    let drain = map.drain();
}

#[test]
fn test_drain_with_empty_map() {
    let mut map: HeaderMap = HeaderMap::with_capacity(0);
    let drain = map.drain();
}

#[test]
fn test_drain_with_max_capacity() {
    let capacity = 32768; // max size as per constraint
    let mut map = HeaderMap::with_capacity(capacity);
    for i in 0..capacity {
        map.insert(HeaderName::from_static(&format!("Key-{}", i)), format!("Value-{}", i).parse().unwrap());
    }
    let drain = map.drain();
}

#[test]
fn test_drain_with_varied_entries() {
    let mut map = HeaderMap::with_capacity(5);
    map.insert(HeaderName::from_static("Key1"), "Value1".parse().unwrap());
    map.append(HeaderName::from_static("Key1"), "Value1-Second".parse().unwrap());
    map.insert(HeaderName::from_static("Key2"), "Value2".parse().unwrap());

    let drain = map.drain();
}

