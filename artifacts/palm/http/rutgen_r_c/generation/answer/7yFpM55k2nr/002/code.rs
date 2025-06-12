// Answer 0

#[test]
fn test_clear_empty_map() {
    let mut map: HeaderMap<String> = HeaderMap::with_capacity(10);
    map.clear();
    assert!(map.is_empty());
    assert!(map.capacity() > 0);
}

#[test]
fn test_clear_non_empty_map() {
    let mut map: HeaderMap<&str> = HeaderMap::with_capacity(10);
    map.insert("Key1", "Value1");
    map.insert("Key2", "Value2");
    
    map.clear();
    assert!(map.is_empty());
    assert!(map.capacity() > 0);
}

#[test]
fn test_clear_map_with_capacity() {
    let mut map: HeaderMap<i32> = HeaderMap::with_capacity(1024);
    map.insert(1, 100);
    map.insert(2, 200);
    
    map.clear();
    assert!(map.is_empty());
    assert!(map.capacity() >= 1024);
}

#[test]
fn test_clear_map_with_no_entries() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(5);
    
    map.clear();
    assert!(map.is_empty());
    assert!(map.capacity() > 0);
}

