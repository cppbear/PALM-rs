// Answer 0

#[test]
fn test_with_capacity_in_non_zero_capacity() {
    use bumpalo::Bump;
    
    let bump = Bump::new();
    let mut map = HashMap::with_capacity_in(5, &bump);

    assert_eq!(map.len(), 0);
    let empty_map_capacity = map.capacity();
    assert!(empty_map_capacity >= 5);
    
    map.insert("One", 1);
    map.insert("Two", 2);
    map.insert("Three", 3);
    map.insert("Four", 4);
    map.insert("Five", 5);
    
    assert_eq!(map.len(), 5);
    assert_eq!(map.capacity(), empty_map_capacity);
}

#[test]
fn test_with_capacity_in_zero_capacity() {
    use bumpalo::Bump;
    
    let bump = Bump::new();
    let mut map = HashMap::with_capacity_in(0, &bump);

    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_with_capacity_in_large_capacity() {
    use bumpalo::Bump;
    
    let bump = Bump::new();
    let large_capacity = 1_000_000;
    let mut map = HashMap::with_capacity_in(large_capacity, &bump);

    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= large_capacity);
}

