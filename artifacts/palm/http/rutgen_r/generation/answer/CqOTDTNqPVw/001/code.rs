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

#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_or_insert_with_panic_on_size_overflow() {
    use http::HeaderMap;
    
    let mut map = HeaderMap::new();
    // Insert enough entries to exceed the maximum size
    for i in 0..=http::MAX_SIZE {
        map.try_insert(format!("key{}", i).as_str(), "value").unwrap();
    }
    
    let _ = map.entry("x-overflow")
        .or_insert_with(|| "should not succeed".parse().unwrap());
}

