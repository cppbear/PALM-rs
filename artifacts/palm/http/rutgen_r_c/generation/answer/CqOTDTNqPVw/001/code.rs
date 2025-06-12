// Answer 0

#[test]
fn test_or_insert_with_insert_empty() {
    struct TestHeaderValue(String);
    
    let mut map = http::HeaderMap::new();
    let res = map.entry("x-hello")
        .or_insert_with(|| TestHeaderValue("world".to_string()));

    assert_eq!(res.0, "world".to_string());
}

#[test]
fn test_or_insert_with_existing_entry() {
    struct TestHeaderValue(String);

    let mut map = http::HeaderMap::new();
    map.try_insert("host", TestHeaderValue("world".to_string())).unwrap();

    let res = map.entry("host")
        .or_insert_with(|| TestHeaderValue("unreachable".to_string()));

    assert_eq!(res.0, "world".to_string());
}

#[test]
#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_or_insert_with_panic_on_max_size_exceed() {
    struct TestHeaderValue(String);

    let mut map = http::HeaderMap::new();
    for i in 0..(http::MAX_SIZE + 1) {
        map.try_insert(format!("key-{}", i), TestHeaderValue(format!("value-{}", i))).unwrap();
    }

    // This should panic because we exceeded the maximum size
    let _ = map.entry("extra-key")
        .or_insert_with(|| TestHeaderValue("new value".to_string()));
}

