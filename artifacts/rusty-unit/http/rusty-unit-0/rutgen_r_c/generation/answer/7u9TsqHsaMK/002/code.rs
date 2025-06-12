// Answer 0

#[test]
fn test_drain_empty_map() {
    let mut map: HeaderMap = HeaderMap::with_capacity(0);
    let drain = map.drain();
    assert_eq!(drain.len, 0);
    assert_eq!(drain.next, None);
}

#[test]
fn test_drain_single_entry() {
    let mut map: HeaderMap = HeaderMap::with_capacity(1);
    let key = "Test-Key".parse::<HeaderName>().unwrap();
    let value = "Test-Value".parse::<HeaderValue>().unwrap();
    map.insert(key.clone(), value.clone());

    let mut drain = map.drain();
    assert_eq!(drain.next(), Some((Some(key), value)));
    assert_eq!(drain.next(), None);
}

#[test]
fn test_drain_multiple_entries() {
    let mut map: HeaderMap = HeaderMap::with_capacity(2);
    let key1 = "Key1".parse::<HeaderName>().unwrap();
    let value1 = "Value1".parse::<HeaderValue>().unwrap();
    let key2 = "Key2".parse::<HeaderName>().unwrap();
    let value2 = "Value2".parse::<HeaderValue>().unwrap();

    map.insert(key1.clone(), value1.clone());
    map.append(key1.clone(), value2.clone());

    let mut drain = map.drain();
    assert_eq!(drain.next(), Some((Some(key1), value1)));
    assert_eq!(drain.next(), Some((None, value2)));
    assert_eq!(drain.next(), None);
}

#[test]
fn test_drain_with_different_keys() {
    let mut map: HeaderMap = HeaderMap::with_capacity(2);
    let key1 = "Host".parse::<HeaderName>().unwrap();
    let value1 = "example.com".parse::<HeaderValue>().unwrap();
    let key2 = "Content-Length".parse::<HeaderName>().unwrap();
    let value2 = "123".parse::<HeaderValue>().unwrap();

    map.insert(key1.clone(), value1.clone());
    map.insert(key2.clone(), value2.clone());

    let mut drain = map.drain();
    assert_eq!(drain.next(), Some((Some(key1), value1)));
    assert_eq!(drain.next(), Some((Some(key2), value2)));
    assert_eq!(drain.next(), None);
}

