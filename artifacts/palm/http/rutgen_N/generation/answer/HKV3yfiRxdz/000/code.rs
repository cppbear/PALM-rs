// Answer 0

#[test]
fn test_get_all_with_existing_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "goodbye".parse().unwrap());

    let view = map.get_all(HOST);
    let mut iter = view.iter();
    assert_eq!(&"hello", iter.next().unwrap());
    assert_eq!(&"goodbye", iter.next().unwrap());
    assert!(iter.next().is_none());
}

#[test]
fn test_get_all_with_non_existing_key() {
    use http::HeaderMap;
    use http::header::USER_AGENT;

    let map = HeaderMap::new();
    let view = map.get_all(USER_AGENT);
    assert!(view.iter().next().is_none());
}

