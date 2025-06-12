// Answer 0

#[test]
fn test_iter_empty_map() {
    let map: HeaderMap = HeaderMap::with_capacity(10);
    let iter = map.iter();
}

#[test]
fn test_iter_single_entry() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("Content-Type".parse().unwrap(), "text/html".parse().unwrap());
    let iter = map.iter();
}

#[test]
fn test_iter_multiple_entries_same_key() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("Content-Type".parse().unwrap(), "text/html".parse().unwrap());
    map.append("Content-Type".parse().unwrap(), "application/json".parse().unwrap());
    let iter = map.iter();
}

#[test]
fn test_iter_multiple_unique_entries() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    map.insert("Content-Type".parse().unwrap(), "text/html".parse().unwrap());
    map.insert("Content-Length".parse().unwrap(), "123".parse().unwrap());
    map.insert("Accept".parse().unwrap(), "application/json".parse().unwrap());
    let iter = map.iter();
}

#[test]
fn test_iter_large_map() {
    let mut map: HeaderMap = HeaderMap::with_capacity(32768);
    for i in 0..32768 {
        map.insert(format!("Key{}", i).parse().unwrap(), format!("Value{}", i).parse().unwrap());
    }
    let iter = map.iter();
}

