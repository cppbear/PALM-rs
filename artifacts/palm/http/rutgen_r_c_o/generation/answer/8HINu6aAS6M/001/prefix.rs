// Answer 0

#[test]
fn test_iter_single_entry() {
    let mut map = HeaderMap::with_capacity(2);
    map.insert("Host", "world".parse().unwrap());
    
    if let Entry::Occupied(e) = map.entry("Host") {
        let mut iter = e.iter();
        let _ = iter.next();
    }
}

#[test]
fn test_iter_multiple_entries() {
    let mut map = HeaderMap::with_capacity(3);
    map.insert("Host", "world".parse().unwrap());
    map.append("Host", "earth".parse().unwrap());
    
    if let Entry::Occupied(e) = map.entry("Host") {
        let mut iter = e.iter();
        let _ = iter.next();
        let _ = iter.next();
    }
}

#[test]
fn test_iter_no_entries() {
    let mut map = HeaderMap::with_capacity(1);
    
    if let Entry::Vacant(_) = map.entry("Non-Existent") {
        // No values to iterate
    }
}

#[test]
fn test_iter_with_empty_header_map() {
    let mut map = HeaderMap::with_capacity(0);
    
    if let Entry::Vacant(_) = map.entry("Host") {
        // No values to iterate
    }
}

#[test]
fn test_iter_multiple_values() {
    let mut map = HeaderMap::with_capacity(10);
    map.insert("User-Agent", "Mozilla/5.0".parse().unwrap());
    map.append("User-Agent", "Chrome/58.0".parse().unwrap());
    map.append("User-Agent", "Safari/537.36".parse().unwrap());

    if let Entry::Occupied(e) = map.entry("User-Agent") {
        let mut iter = e.iter();
        let _ = iter.next();
        let _ = iter.next();
        let _ = iter.next();
    }
}

#[test]
fn test_iter_edge_case() {
    let mut map = HeaderMap::with_capacity(5);
    map.insert("Accept", "text/html".parse().unwrap());
    
    if let Entry::Occupied(e) = map.entry("Accept") {
        let mut iter = e.iter();
        let single_value = iter.next();
        assert!(single_value.is_some());
        let second_value = iter.next();
        assert!(second_value.is_none());
    }
}

