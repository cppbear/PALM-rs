// Answer 0

#[test]
fn test_append_new_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    assert!(map.append(HOST, "world".parse().unwrap()).is_none());
    assert!(!map.is_empty());
}

#[test]
fn test_append_existing_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.append(HOST, "world".parse().unwrap());
    assert!(!map.is_empty());

    map.append(HOST, "earth".parse().unwrap());
    let values = map.get_all("host");
    let mut i = values.iter();
    assert_eq!("world", *i.next().unwrap());
    assert_eq!("earth", *i.next().unwrap());
}

#[should_panic]
fn test_append_exceed_capacity() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    // Assuming MAX_SIZE is 10 for the purpose of this test
    for i in 0..11 {
        map.append(HOST, format!("value{}", i).parse().unwrap());
    }
}

