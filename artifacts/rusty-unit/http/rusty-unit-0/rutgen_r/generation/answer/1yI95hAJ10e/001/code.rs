// Answer 0

#[test]
fn test_is_empty_when_initialized() {
    let map = http::HeaderMap::new();
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_after_insert() {
    let mut map = http::HeaderMap::new();
    map.insert(http::header::HOST, "hello.world".parse().unwrap());
    assert!(!map.is_empty());
}

#[test]
fn test_is_empty_after_multiple_inserts() {
    let mut map = http::HeaderMap::new();
    map.insert(http::header::HOST, "example.com".parse().unwrap());
    map.insert(http::header::USER_AGENT, "test-agent".parse().unwrap());
    assert!(!map.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    let mut map = http::HeaderMap::new();
    map.insert(http::header::HOST, "example.com".parse().unwrap());
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_on_non_empty_map() {
    let mut map = http::HeaderMap::new();
    map.insert(http::header::HOST, "example.com".parse().unwrap());
    assert!(!map.is_empty());
}

