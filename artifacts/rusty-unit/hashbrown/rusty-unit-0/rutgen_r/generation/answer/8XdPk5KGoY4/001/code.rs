// Answer 0

#[test]
fn test_with_capacity_in_zero_capacity() {
    use hashbrown::HashMap;
    use bumpalo::Bump;

    let bump = Bump::new();
    let map: HashMap<&str, i32> = HashMap::with_capacity_in(0, &bump);

    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 0);
}

#[test]
fn test_with_capacity_in_non_zero_capacity() {
    use hashbrown::HashMap;
    use bumpalo::Bump;

    let bump = Bump::new();
    let map = HashMap::with_capacity_in(5, &bump);

    assert_eq!(map.len(), 0);
    let empty_map_capacity = map.capacity();
    assert!(empty_map_capacity >= 5);
}

#[test]
fn test_with_capacity_in_insert_elements() {
    use hashbrown::HashMap;
    use bumpalo::Bump;

    let bump = Bump::new();
    let mut map = HashMap::with_capacity_in(5, &bump);

    map.insert("One", 1);
    map.insert("Two", 2);
    map.insert("Three", 3);
    map.insert("Four", 4);
    map.insert("Five", 5);

    assert_eq!(map.len(), 5);
    let map_capacity_after_insert = map.capacity();
    assert!(map_capacity_after_insert >= 5);
}

#[test]
fn test_with_capacity_in_boundary_conditions() {
    use hashbrown::HashMap;
    use bumpalo::Bump;

    let bump = Bump::new();
    let mut map = HashMap::with_capacity_in(1, &bump);

    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 1);

    map.insert("Single", 1);
    assert_eq!(map.len(), 1);
}

#[should_panic]
fn test_with_capacity_in_panic_condition() {
    use hashbrown::HashMap;
    use bumpalo::Bump;

    let bump = Bump::new();
    let mut map = HashMap::with_capacity_in(5, &bump);

    // Attempting to panic by exceeding initial capacity
    for i in 0..10 {
        map.insert(&format!("Key{}", i), i as i32);
    }
}

