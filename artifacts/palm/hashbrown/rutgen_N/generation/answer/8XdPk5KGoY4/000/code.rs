// Answer 0

#[test]
fn test_with_capacity_in_non_zero() {
    use hashbrown::HashMap;
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
fn test_with_capacity_in_zero() {
    use hashbrown::HashMap;
    use bumpalo::Bump;

    let bump = Bump::new();
    let map = HashMap::with_capacity_in(0, &bump);

    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_with_capacity_in_boundary() {
    use hashbrown::HashMap;
    use bumpalo::Bump;

    let bump = Bump::new();
    let mut map = HashMap::with_capacity_in(1, &bump);

    assert_eq!(map.len(), 0);
    let empty_map_capacity = map.capacity();
    assert!(empty_map_capacity >= 1);

    map.insert("One", 1);

    assert_eq!(map.len(), 1);
    assert_eq!(map.capacity(), empty_map_capacity);
}

