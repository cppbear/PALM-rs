// Answer 0

#[test]
fn test_hash_map_capacity_empty() {
    let map: HashMap<i32, i32> = HashMap::with_capacity(10);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 10);
}

#[test]
fn test_hash_map_capacity_non_empty() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(20);
    map.insert(1, 10);
    map.insert(2, 20);
    assert_eq!(map.len(), 2);
    assert!(map.capacity() >= 20);
}

#[test]
fn test_hash_map_capacity_large() {
    let map: HashMap<u64, u64> = HashMap::with_capacity(1_000_000);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 1_000_000);
}

#[test]
fn test_hash_map_capacity_zero() {
    let map: HashMap<i32, i32> = HashMap::with_capacity(0);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 0);
}

