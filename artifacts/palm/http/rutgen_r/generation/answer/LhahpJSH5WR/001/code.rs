// Answer 0

#[test]
fn test_keys_empty_header_map() {
    use http::HeaderMap;

    let map = HeaderMap::new();
    let keys: Vec<_> = map.keys().collect();
    assert!(keys.is_empty());
}

#[test]
fn test_keys_single_entry() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH};

    let mut map = HeaderMap::new();
    map.insert(CONTENT_LENGTH, "123".parse().unwrap());
    
    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0], CONTENT_LENGTH);
}

#[test]
fn test_keys_multiple_entries_same_key() {
    use http::HeaderMap;
    use http::header::{HOST};

    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "goodbye".parse().unwrap());

    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0], HOST);
}

#[test]
fn test_keys_multiple_unique_entries() {
    use http::HeaderMap;
    use http::header::{HOST, CONTENT_LENGTH};

    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    map.insert(CONTENT_LENGTH, "123".parse().unwrap());

    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&HOST));
    assert!(keys.contains(&CONTENT_LENGTH));
}

#[test]
fn test_keys_order_consistency() {
    use http::HeaderMap;
    use http::header::{HOST, CONTENT_LENGTH};

    let mut map = HeaderMap::new();
    map.insert(CONTENT_LENGTH, "123".parse().unwrap());
    map.insert(HOST, "hello".parse().unwrap());

    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&CONTENT_LENGTH));
    assert!(keys.contains(&HOST));
}

