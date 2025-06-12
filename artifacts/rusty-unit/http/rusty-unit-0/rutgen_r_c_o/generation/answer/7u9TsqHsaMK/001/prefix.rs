// Answer 0

#[test]
fn test_drain_empty_map() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(0);
    let drain = map.drain();
}

#[test]
fn test_drain_single_entry() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(1);
    map.insert(HeaderName::from("Host"), "localhost".parse().unwrap());
    let drain = map.drain();
}

#[test]
fn test_drain_multiple_entries() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(3);
    map.insert(HeaderName::from("Host"), "localhost".parse().unwrap());
    map.append(HeaderName::from("Host"), "127.0.0.1".parse().unwrap());
    map.insert(HeaderName::from("Content-Length"), "100".parse().unwrap());
    let drain = map.drain();
}

#[test]
fn test_drain_invoke_twice() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(2);
    map.insert(HeaderName::from("Content-Type"), "text/html".parse().unwrap());
    let first_drain = map.drain();
    let second_drain = map.drain();
}

#[test]
fn test_drain_large_entry_size() {
    let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(32768);
    for i in 0..32768 {
        map.insert(HeaderName::from(format!("Header-{}", i)), "value".parse().unwrap());
    }
    let drain = map.drain();
}

