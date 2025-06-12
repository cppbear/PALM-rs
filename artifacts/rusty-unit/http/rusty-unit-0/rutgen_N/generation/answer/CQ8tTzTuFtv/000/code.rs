// Answer 0

#[test]
fn test_header_map_iter_empty() {
    use http::HeaderMap;
    let map = HeaderMap::new();
    let entries: Vec<_> = map.iter().collect();
    assert!(entries.is_empty());
}

#[test]
fn test_header_map_iter_single_entry() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH, HOST};
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());

    let entries: Vec<_> = map.iter().collect();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, HOST);
    assert_eq!(entries[0].1, "hello".parse().unwrap());
}

#[test]
fn test_header_map_iter_multiple_entries() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH, HOST};
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "goodbye".parse().unwrap());
    map.insert(CONTENT_LENGTH, "123".parse().unwrap());

    let entries: Vec<_> = map.iter().collect();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].0, HOST);
    assert_eq!(entries[0].1, "hello".parse().unwrap());
    assert_eq!(entries[1].0, HOST);
    assert_eq!(entries[1].1, "goodbye".parse().unwrap());
    assert_eq!(entries[2].0, CONTENT_LENGTH);
    assert_eq!(entries[2].1, "123".parse().unwrap());
}

#[test]
fn test_header_map_iter_order() {
    use http::HeaderMap;
    use http::header::{HOST, USER_AGENT};
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "example.com".parse().unwrap());
    map.append(USER_AGENT, "test-agent".parse().unwrap());
    map.append(USER_AGENT, "test-agent-2".parse().unwrap());

    let entries: Vec<_> = map.iter().collect();
    assert_eq!(entries.len(), 3);
    assert!(entries.contains(&(&HOST, "example.com".parse().unwrap())));
    assert!(entries.contains(&(&USER_AGENT, "test-agent".parse().unwrap())));
    assert!(entries.contains(&(&USER_AGENT, "test-agent-2".parse().unwrap())));
}

