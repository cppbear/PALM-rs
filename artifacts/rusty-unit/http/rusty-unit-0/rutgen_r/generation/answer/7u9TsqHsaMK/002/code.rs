// Answer 0

#[test]
fn test_drain_empty_header_map() {
    let mut map = http::HeaderMap::new();
    let mut drain = map.drain();

    assert_eq!(drain.next(), None);
}

#[test]
fn test_drain_single_entry() {
    let mut map = http::HeaderMap::new();
    let host = http::header::HOST;
    map.insert(host, "hello".parse().unwrap());
    let mut drain = map.drain();

    assert_eq!(drain.next(), Some((Some(host), "hello".parse().unwrap())));
    assert_eq!(drain.next(), None);
}

#[test]
fn test_drain_multiple_entries_same_header() {
    let mut map = http::HeaderMap::new();
    let host = http::header::HOST;
    map.insert(host, "hello".parse().unwrap());
    map.append(host, "goodbye".parse().unwrap());
    let mut drain = map.drain();

    assert_eq!(drain.next(), Some((Some(host), "hello".parse().unwrap())));
    assert_eq!(drain.next(), Some((None, "goodbye".parse().unwrap())));
    assert_eq!(drain.next(), None);
}

#[test]
fn test_drain_multiple_different_headers() {
    let mut map = http::HeaderMap::new();
    let host = http::header::HOST;
    let content_length = http::header::CONTENT_LENGTH;

    map.insert(host, "hello".parse().unwrap());
    map.insert(content_length, "123".parse().unwrap());
    
    let mut drain = map.drain();

    assert_eq!(drain.next(), Some((Some(host), "hello".parse().unwrap())));
    assert_eq!(drain.next(), Some((Some(content_length), "123".parse().unwrap())));
    assert_eq!(drain.next(), None);
}

#[test]
fn test_drain_with_extra_values() {
    let mut map = http::HeaderMap::new();
    let host = http::header::HOST;
    map.insert(host, "hello".parse().unwrap());
    map.append(host, "goodbye".parse().unwrap());
    
    // Simulating an additional state without actual implementation of extra_values
    // Assuming that the drain function should handle associated internal values correctly
    
    let mut drain = map.drain();

    assert_eq!(drain.next(), Some((Some(host), "hello".parse().unwrap())));
    assert_eq!(drain.next(), Some((None, "goodbye".parse().unwrap())));
    assert_eq!(drain.next(), None);
}

