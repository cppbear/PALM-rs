// Answer 0

#[test]
fn test_rebuild_empty_map() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(0);
    assert_eq!(map.len(), 0);
    map.rebuild(); // Should not panic
}

#[test]
fn test_rebuild_single_entry() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(1);
    map.insert("key1", 42);
    assert_eq!(map.len(), 1);
    map.rebuild(); // Should not panic
    assert_eq!(map.get("key1"), Some(&42));
}

#[test]
fn test_rebuild_multiple_entries() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(5);
    map.insert("key1", 42);
    map.insert("key2", 84);
    map.insert("key3", 21);
    assert_eq!(map.len(), 3);
    map.rebuild(); // Should not panic
    assert_eq!(map.get("key1"), Some(&42));
    assert_eq!(map.get("key2"), Some(&84));
    assert_eq!(map.get("key3"), Some(&21));
}

#[test]
fn test_rebuild_with_collision() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(5);
    map.insert("key1", 42);
    // Simulate a collision by inserting a key that results in the same hash.
    map.insert("key1_collision", 84); // Assuming it collides with "key1"
    assert_eq!(map.len(), 2);
    map.rebuild(); // Should not panic
    assert_eq!(map.get("key1"), Some(&42));
    assert_eq!(map.get("key1_collision"), Some(&84));
}

#[test]
#[should_panic]
fn test_rebuild_large_map_beyond_capacity() {
    let mut map: HeaderMap<u32> = HeaderMap::try_with_capacity(32).unwrap();
    for i in 0..32 {
        map.insert(format!("key{}", i), i);
    }
    assert_eq!(map.len(), 32);
    map.rebuild(); // Should trigger a panic if the structure exceeds any constraints
}

