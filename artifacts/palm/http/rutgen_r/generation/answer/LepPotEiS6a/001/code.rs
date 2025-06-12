// Answer 0

#[test]
fn test_keys_len_empty_map() {
    let map = http::HeaderMap::new();
    assert_eq!(0, map.keys_len());
}

#[test]
fn test_keys_len_single_insert() {
    let mut map = http::HeaderMap::new();
    map.insert(http::header::ACCEPT, "text/plain".parse().unwrap());
    assert_eq!(1, map.keys_len());
}

#[test]
fn test_keys_len_multiple_different_keys() {
    let mut map = http::HeaderMap::new();
    map.insert(http::header::ACCEPT, "text/plain".parse().unwrap());
    map.insert(http::header::HOST, "localhost".parse().unwrap());
    assert_eq!(2, map.keys_len());
}

#[test]
fn test_keys_len_same_key_multiple_values() {
    let mut map = http::HeaderMap::new();
    map.insert(http::header::ACCEPT, "text/plain".parse().unwrap());
    map.insert(http::header::ACCEPT, "text/html".parse().unwrap());
    assert_eq!(1, map.keys_len());
}

#[test]
fn test_keys_len_no_repeats() {
    let mut map = http::HeaderMap::new();
    map.insert(http::header::ACCEPT, "text/plain".parse().unwrap());
    map.insert(http::header::HOST, "localhost".parse().unwrap());
    map.insert(http::header::USER_AGENT, "test-agent".parse().unwrap());
    assert_eq!(3, map.keys_len());
}

#[test]
fn test_keys_len_repeated_keys_no_panic() {
    let mut map = http::HeaderMap::new();
    for _ in 0..100 {
        map.insert(http::header::USER_AGENT, "test-agent".parse().unwrap());
    }
    assert_eq!(1, map.keys_len());
}

