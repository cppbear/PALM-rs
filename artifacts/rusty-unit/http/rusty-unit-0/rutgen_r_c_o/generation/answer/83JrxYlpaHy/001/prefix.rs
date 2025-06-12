// Answer 0

#[test]
fn test_get_valid_entry() {
    let key = HeaderName::from_static("host");
    let mut map = HeaderMap::new();
    map.insert(key.clone(), "hello.world".parse().unwrap());
    
    if let Entry::Occupied(e) = map.entry("host") {
        let value = e.get();
    }
}

#[should_panic]
fn test_get_empty_entry() {
    let key = HeaderName::from_static("host");
    let mut map = HeaderMap::new();
    
    if let Entry::Occupied(e) = map.entry("host") {
        let value = e.get();
    }
}

#[test]
fn test_get_entry_with_multiple_values() {
    let key = HeaderName::from_static("host");
    let mut map = HeaderMap::new();
    map.insert(key.clone(), "hello.world".parse().unwrap());
    if let Entry::Occupied(mut e) = map.entry("host") {
        e.append("hello.earth".parse().unwrap());
        
        let value = e.get();
    }
}

#[test]
fn test_get_different_key() {
    let key1 = HeaderName::from_static("host");
    let key2 = HeaderName::from_static("user-agent");
    let mut map = HeaderMap::new();
    map.insert(key1.clone(), "hello.world".parse().unwrap());
    map.insert(key2.clone(), "curl/7.68.0".parse().unwrap());

    if let Entry::Occupied(e) = map.entry("user-agent") {
        let value = e.get();
    }
}

