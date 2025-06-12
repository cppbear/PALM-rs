// Answer 0

#[test]
fn test_or_try_insert_with_into_vacant_entry() {
    use http::HeaderMap;
    use http::header::HeaderName;

    let mut map = HeaderMap::new();
    
    let res = map.entry(HeaderName::from_static("x-hello"))
        .or_try_insert_with(|| "world".parse().unwrap()).unwrap();

    assert_eq!(res, "world");
}

#[test]
fn test_or_try_insert_with_into_occupied_entry() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.try_insert(HOST, "world".parse().unwrap()).unwrap();

    let res = map.entry(HOST)
        .or_try_insert_with(|| unreachable!()).unwrap();

    assert_eq!(res, "world");
}

#[test]
#[should_panic]
fn test_or_try_insert_with_with_unreachable_panic() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.try_insert(HOST, "world".parse().unwrap()).unwrap();

    // This should panic because the entry exists
    let _res = map.entry(HOST)
        .or_try_insert_with(|| unreachable!()).unwrap();
}

