// Answer 0

#[test]
fn test_insert_new_key() {
    use http::HeaderMap;
    use http::header::USER_AGENT;

    let mut map = HeaderMap::new();
    let result = map.insert(USER_AGENT, "Mozilla/5.0".parse().unwrap());
    assert!(result.is_none());
    assert!(!map.is_empty());
}

#[test]
fn test_insert_existing_key() {
    use http::HeaderMap;
    use http::header::USER_AGENT;

    let mut map = HeaderMap::new();
    let _ = map.insert(USER_AGENT, "Mozilla/5.0".parse().unwrap());
    let prev = map.insert(USER_AGENT, "Chrome/90.0".parse().unwrap()).unwrap();
    
    assert_eq!("Mozilla/5.0", prev);
    assert_eq!(1, map.len());
}

#[test]
#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_insert_exceeding_capacity() {
    use http::HeaderMap;
    use http::header::USER_AGENT;

    let mut map = HeaderMap::with_capacity(http::MAX_SIZE);
    let _ = map.insert(USER_AGENT, "Initial".parse().unwrap());

    for _ in 0..http::MAX_SIZE {
        map.insert(USER_AGENT, "Another".parse().unwrap());
    }
}

#[test]
fn test_insert_different_values() {
    use http::HeaderMap;
    use http::header::USER_AGENT;

    let mut map = HeaderMap::new();
    let _ = map.insert(USER_AGENT, "Mozilla/5.0".parse().unwrap());
    let prev = map.insert(USER_AGENT, "Safari/14.0".parse().unwrap()).unwrap();

    assert_eq!("Mozilla/5.0", prev);
    assert_eq!(1, map.len());
}

#[test]
fn test_insert_multiple_keys() {
    use http::HeaderMap;
    use http::header::{USER_AGENT, HOST};

    let mut map = HeaderMap::new();
    let user_agent_prev = map.insert(USER_AGENT, "Mozilla/5.0".parse().unwrap()).unwrap();
    let host_prev = map.insert(HOST, "localhost".parse().unwrap()).unwrap();

    assert_eq!(1, map.len());
    assert_eq!("localhost", host_prev);
    assert_eq!("Mozilla/5.0", user_agent_prev);
}

