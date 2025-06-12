// Answer 0

#[test]
fn test_clear_on_empty_header_map() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.clear();
    assert!(map.is_empty());
    assert!(map.capacity() > 0);
}

#[test]
fn test_clear_after_inserting_multiple_entries() {
    use http::HeaderMap;
    use http::header::{HOST, USER_AGENT};

    let mut map = HeaderMap::new();
    map.insert(HOST, "example.com".parse().unwrap());
    map.insert(USER_AGENT, "test-agent".parse().unwrap());

    map.clear();
    assert!(map.is_empty());
    assert!(map.capacity() > 0);
}

#[test]
fn test_clear_on_large_header_map() {
    use http::HeaderMap;
    use http::header::{HOST, CONTENT_TYPE};

    let mut map = HeaderMap::new();
    for i in 0..1000 {
        map.insert(
            HOST,
            format!("example{}.com", i).parse().unwrap(),
        );
        map.insert(
            CONTENT_TYPE,
            format!("text/plain; charset=utf-{}", i).parse().unwrap(),
        );
    }

    map.clear();
    assert!(map.is_empty());
    assert!(map.capacity() > 0);
}

#[test]
fn test_clear_on_header_map_with_empty_entries() {
    use http::HeaderMap;

    let mut map = HeaderMap::new();
    map.clear(); // Already empty, should not panic
    assert!(map.is_empty());
    assert!(map.capacity() > 0);
}

