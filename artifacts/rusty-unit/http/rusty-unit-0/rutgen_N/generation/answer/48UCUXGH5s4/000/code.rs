// Answer 0

#[test]
fn test_insert_value_into_entry() {
    use http::header::{HeaderMap, Entry};
    use std::net::SocketAddr; // Assuming we are using this as a type for demonstration

    let mut map = HeaderMap::new();

    if let Entry::Vacant(v) = map.entry("x-hello") {
        v.insert("world".parse::<SocketAddr>().unwrap());
    }

    assert_eq!(map["x-hello"].to_string(), "world");
}

#[test]
fn test_insert_value_overflow() {
    use http::header::{HeaderMap, Entry};
    use std::net::SocketAddr;

    let mut map = HeaderMap::new();

    for i in 0..=HeaderMap::MAX_SIZE {
        if let Entry::Vacant(v) = map.entry(format!("x-hello-{}", i)) {
            v.insert("world".parse::<SocketAddr>().unwrap());
        }
    }

    assert!(map.entry("x-hello-overflow").is_err());
}

