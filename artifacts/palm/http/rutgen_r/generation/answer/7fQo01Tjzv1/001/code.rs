// Answer 0

#[test]
fn test_try_append_new_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    assert!(map.try_append(HOST, "world".parse().unwrap()).is_ok());
    assert!(!map.is_empty());
}

#[test]
fn test_try_append_existing_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.try_append(HOST, "world".parse().unwrap()).unwrap();

    assert!(map.try_append(HOST, "earth".parse().unwrap()).is_ok());

    let values = map.get_all("host");
    let mut i = values.iter();
    assert_eq!("world", *i.next().unwrap());
    assert_eq!("earth", *i.next().unwrap());
}

#[test]
fn test_try_append_exceed_capacity() {
    // Assuming a hypothetical max capacity for demonstration.
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();

    // Insert maximum allowable values. Replace `MAX_CAPACITY` with an appropriate number
    for i in 1..=MAX_CAPACITY {
        assert!(map.try_append(HOST, format!("value{}", i).parse().unwrap()).is_ok());
    }

    // Now try to insert one more value which should exceed the capacity
    let result = map.try_append(HOST, "overflow".parse().unwrap());
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_try_append_invalid_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    // This test assumes that an invalid key (non-header name) leads to a panic
    let invalid_key = ""; // Simulating an invalid header key

    let _ = map.try_append(invalid_key.parse().unwrap(), "value".parse().unwrap());
}

#[test]
fn test_try_append_multiple_values() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.try_append(HOST, "first".parse().unwrap()).unwrap();
    map.try_append(HOST, "second".parse().unwrap()).unwrap();
    map.try_append(HOST, "third".parse().unwrap()).unwrap();

    let values = map.get_all("host");
    let mut i = values.iter();
    assert_eq!("first", *i.next().unwrap());
    assert_eq!("second", *i.next().unwrap());
    assert_eq!("third", *i.next().unwrap());
}

