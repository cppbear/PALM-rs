// Answer 0

#[test]
fn test_drain_single_insert() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH, HOST};

    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());

    let mut drain = map.drain();

    assert_eq!(drain.next(), Some((Some(HOST), "hello".parse().unwrap())));
    assert_eq!(drain.next(), None);
}

#[test]
fn test_drain_multiple_appends() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH, HOST};

    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "goodbye".parse().unwrap());
    map.insert(CONTENT_LENGTH, "123".parse().unwrap());

    let mut drain = map.drain();

    assert_eq!(drain.next(), Some((Some(HOST), "hello".parse().unwrap())));
    assert_eq!(drain.next(), Some((None, "goodbye".parse().unwrap())));
    assert_eq!(drain.next(), Some((Some(CONTENT_LENGTH), "123".parse().unwrap())));
    assert_eq!(drain.next(), None);
}

#[test]
fn test_drain_empty() {
    use http::HeaderMap;

    let mut map = HeaderMap::new();
    let mut drain = map.drain();

    assert_eq!(drain.next(), None);
}

