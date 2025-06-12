// Answer 0

#[test]
fn test_len_empty_header_map() {
    let map: HeaderMap = HeaderMap::with_capacity(0);
    assert_eq!(0, map.len());
}

#[test]
fn test_len_single_insert() {
    let mut map: HeaderMap = HeaderMap::with_capacity(1);
    map.insert("Test-Key", "Test-Value").unwrap();
    assert_eq!(1, map.len());
}

#[test]
fn test_len_multiple_inserts() {
    let mut map: HeaderMap = HeaderMap::with_capacity(5);
    map.insert("Key-1", "Value-1").unwrap();
    map.insert("Key-2", "Value-2").unwrap();
    assert_eq!(2, map.len());
}

#[test]
fn test_len_append_values() {
    let mut map: HeaderMap = HeaderMap::with_capacity(3);
    map.insert("Key-1", "Value-1").unwrap();
    map.append("Key-1", "Value-2").unwrap();
    assert_eq!(2, map.len());
    
    map.append("Key-1", "Value-3").unwrap();
    assert_eq!(3, map.len());
}

#[test]
fn test_len_with_extra_values() {
    let mut map: HeaderMap = HeaderMap::with_capacity(5);
    map.insert("Key-A", "Value-A").unwrap();
    map.append("Key-A", "Value-B").unwrap();
    map.append("Key-A", "Value-C").unwrap();
    assert_eq!(3, map.len());
    
    map.insert("Key-B", "Value-D").unwrap();
    assert_eq!(4, map.len());
}

#[test]
fn test_len_reset_after_clear() {
    let mut map: HeaderMap = HeaderMap::with_capacity(5);
    map.insert("Key-1", "Value-1").unwrap();
    assert_eq!(1, map.len());
    
    map.clear();
    assert_eq!(0, map.len());
}

#[test]
fn test_len_non_univocal_keys() {
    let mut map: HeaderMap = HeaderMap::with_capacity(3);
    map.insert("Key-1", "Value-1").unwrap();
    map.insert("Key-1", "Value-1.1").unwrap(); // Overwriting the same key
    assert_eq!(1, map.len());
    
    map.append("Key-1", "Value-2").unwrap();
    assert_eq!(2, map.len());
}

