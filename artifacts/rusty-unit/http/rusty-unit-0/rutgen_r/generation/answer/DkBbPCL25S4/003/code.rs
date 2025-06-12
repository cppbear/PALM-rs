// Answer 0

#[test]
fn test_remove_key_not_found() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    // Attempt to remove a key that was never inserted into the map
    let result = map.remove(HOST);
    // Expect the result to be None since the key does not exist
    assert!(result.is_none());
}

#[test]
fn test_remove_key_not_found_after_insertion() {
    use http::HeaderMap;
    use http::header::USER_AGENT;

    let mut map = HeaderMap::new();
    map.insert(USER_AGENT, "test-agent".parse().unwrap());

    // Remove a different key that does not exist in the map
    let result = map.remove(HOST);
    // Expect the result to be None since HOST was never inserted
    assert!(result.is_none());
}

