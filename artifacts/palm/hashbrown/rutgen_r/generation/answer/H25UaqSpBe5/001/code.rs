// Answer 0

#[test]
fn test_with_capacity_zero() {
    use hashbrown::HashMap;

    let map: HashMap<&str, i32> = HashMap::with_capacity(0);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() == 0);
}

#[test]
fn test_with_capacity_non_zero() {
    use hashbrown::HashMap;

    let capacity = 10;
    let map: HashMap<&str, i32> = HashMap::with_capacity(capacity);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= capacity);
}

#[test]
fn test_with_capacity_large_value() {
    use hashbrown::HashMap;

    let capacity = 1_000_000;
    let map: HashMap<&str, i32> = HashMap::with_capacity(capacity);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= capacity);
}

#[test]
#[should_panic]
fn test_with_capacity_negative() {
    // Trying to simulate a negative capacity which will not be possible in Rust,
    // so it's assumed to trigger a panic (this case shouldn't compile, but shows intent).
    let _map: HashMap<&str, i32> = HashMap::with_capacity(usize::MAX);
}

#[test]
fn test_with_capacity_boundary() {
    use hashbrown::HashMap;

    let capacity = std::usize::MAX;
    let map: HashMap<&str, i32> = HashMap::with_capacity(capacity);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= capacity);
}

