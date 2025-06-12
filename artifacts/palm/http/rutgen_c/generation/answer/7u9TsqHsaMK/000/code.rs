// Answer 0

#[test]
fn test_drain_empty() {
    let mut map: HeaderMap = HeaderMap::with_capacity(0);
    let mut drain = map.drain();
    
    assert_eq!(drain.next(), None);
}

#[test]
fn test_drain_single_entry() {
    let mut map: HeaderMap = HeaderMap::with_capacity(1);
    map.insert("Host", "hello".parse().unwrap());
    let mut drain = map.drain();
    
    assert_eq!(drain.next(), Some((Some("Host".parse().unwrap()), "hello".parse().unwrap())));
    assert_eq!(drain.next(), None);
}

#[test]
fn test_drain_multiple_entries() {
    let mut map: HeaderMap = HeaderMap::with_capacity(3);
    map.insert("Host", "hello".parse().unwrap());
    map.append("Host", "goodbye".parse().unwrap());
    map.insert("Content-Length", "123".parse().unwrap());

    let mut drain = map.drain();

    assert_eq!(drain.next(), Some((Some("Host".parse().unwrap()), "hello".parse().unwrap())));
    assert_eq!(drain.next(), Some((None, "goodbye".parse().unwrap())));
    assert_eq!(drain.next(), Some((Some("Content-Length".parse().unwrap()), "123".parse().unwrap())));
    assert_eq!(drain.next(), None);
}

#[test]
fn test_drain_with_no_empty_entries() {
    let mut map: HeaderMap = HeaderMap::with_capacity(2);
    map.insert("Key1", "Value1".parse().unwrap());
    map.insert("Key2", "Value2".parse().unwrap());

    let mut drain = map.drain();

    assert_eq!(drain.next(), Some((Some("Key1".parse().unwrap()), "Value1".parse().unwrap())));
    assert_eq!(drain.next(), Some((Some("Key2".parse().unwrap()), "Value2".parse().unwrap())));
    assert_eq!(drain.next(), None);
}

