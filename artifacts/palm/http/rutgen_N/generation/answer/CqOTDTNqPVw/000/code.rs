// Answer 0

#[test]
fn test_or_insert_with_empty_entry() {
    use http::HeaderMap;
    
    let mut map = HeaderMap::new();
    
    let res = map.entry("x-hello")
        .or_insert_with(|| "world".parse().unwrap());
    
    assert_eq!(res, "world");
}

#[test]
fn test_or_insert_with_existing_entry() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.try_insert(HOST, "world".parse().unwrap()).unwrap();

    let res = map.try_entry("host")
        .unwrap()
        .or_try_insert_with(|| unreachable!())
        .unwrap();

    assert_eq!(res, "world");
}

