// Answer 0

#[test]
fn test_keys_len_empty() {
    let map = http::HeaderMap::new();
    assert_eq!(0, map.keys_len());
}

#[test]
fn test_keys_len_single_key() {
    let mut map = http::HeaderMap::new();
    let accept = http::header::ACCEPT;
    map.insert(accept, "text/plain".parse().unwrap());
    assert_eq!(1, map.keys_len());
}

#[test]
fn test_keys_len_multiple_keys() {
    let mut map = http::HeaderMap::new();
    let accept = http::header::ACCEPT;
    let host = http::header::HOST;
    
    map.insert(accept, "text/plain".parse().unwrap());
    map.insert(host, "localhost".parse().unwrap());
    assert_eq!(2, map.keys_len());
}

#[test]
fn test_keys_len_same_key_multiple_values() {
    let mut map = http::HeaderMap::new();
    let accept = http::header::ACCEPT;

    map.insert(accept, "text/plain".parse().unwrap());
    map.insert(accept, "text/html".parse().unwrap());
    assert_eq!(1, map.keys_len());
}

#[test]
fn test_keys_len_no_duplicates() {
    let mut map = http::HeaderMap::new();
    let accept = http::header::ACCEPT;
    let host = http::header::HOST;

    map.insert(accept, "text/plain".parse().unwrap());
    map.insert(host, "localhost".parse().unwrap());
    map.insert(accept, "text/html".parse().unwrap()); // Adding a value for existing key
    assert_eq!(2, map.keys_len()); // Only the number of unique keys
}

