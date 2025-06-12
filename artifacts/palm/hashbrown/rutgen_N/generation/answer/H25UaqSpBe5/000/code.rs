// Answer 0

#[test]
fn test_with_capacity_zero() {
    let map: hashbrown::HashMap<&str, i32> = hashbrown::HashMap::with_capacity(0);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() == 0);
}

#[test]
fn test_with_capacity_non_zero() {
    let capacity = 10;
    let map: hashbrown::HashMap<&str, i32> = hashbrown::HashMap::with_capacity(capacity);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= capacity);
}

#[test]
fn test_with_capacity_small_non_zero() {
    let capacity = 1;
    let map: hashbrown::HashMap<&str, i32> = hashbrown::HashMap::with_capacity(capacity);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= capacity);
}

#[test]
fn test_with_capacity_large_number() {
    let capacity = 1000;
    let map: hashbrown::HashMap<&str, i32> = hashbrown::HashMap::with_capacity(capacity);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= capacity);
}

