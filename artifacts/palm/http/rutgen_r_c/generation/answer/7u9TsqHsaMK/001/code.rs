// Answer 0

#[test]
fn test_drain_with_entries() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH, HOST};
    
    let mut map = HeaderMap::with_capacity(5);
    
    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "goodbye".parse().unwrap());
    map.insert(CONTENT_LENGTH, "123".parse().unwrap());

    let mut drain = map.drain();

    assert_eq!(drain.len, 3);
    assert_eq!(drain.next(), Some((Some(HOST), "hello".parse().unwrap())));
    assert_eq!(drain.next(), Some((None, "goodbye".parse().unwrap())));
    assert_eq!(drain.next(), Some((Some(CONTENT_LENGTH), "123".parse().unwrap())));
    assert_eq!(drain.next(), None);
}

#[test]
fn test_drain_with_no_entries() {
    use http::HeaderMap;

    let mut map = HeaderMap::with_capacity(5);
    
    let mut drain = map.drain();

    assert_eq!(drain.len, 0);
    assert_eq!(drain.next(), None);
}

#[test]
#[should_panic]
fn test_drain_panic_on_empty_entries() {
    use http::HeaderMap;

    let mut map = HeaderMap::with_capacity(0);
    
    let _drain = map.drain(); // This might panic under certain conditions based on implementation.
}

