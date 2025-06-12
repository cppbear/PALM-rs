// Answer 0

#[test]
fn test_with_capacity_and_hasher_zero_capacity() {
    use hashbrown::HashMap;
    use hashbrown::DefaultHashBuilder;

    let s = DefaultHashBuilder::default();
    let map: HashMap<_, _, _> = HashMap::with_capacity_and_hasher(0, s);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() == 0);
}

#[test]
fn test_with_capacity_and_hasher_non_zero_capacity() {
    use hashbrown::HashMap;
    use hashbrown::DefaultHashBuilder;

    let s = DefaultHashBuilder::default();
    let map: HashMap<_, _, _> = HashMap::with_capacity_and_hasher(10, s);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 10);
}

#[test]
fn test_with_capacity_and_hasher_large_capacity() {
    use hashbrown::HashMap;
    use hashbrown::DefaultHashBuilder;

    let s = DefaultHashBuilder::default();
    let map: HashMap<_, _, _> = HashMap::with_capacity_and_hasher(1_000_000, s);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 1_000_000);
}

#[should_panic]
fn test_with_capacity_and_hasher_negative_capacity() {
    use hashbrown::HashMap;
    use hashbrown::DefaultHashBuilder;

    let s = DefaultHashBuilder::default();
    let _map: HashMap<_, _, _> = HashMap::with_capacity_and_hasher(usize::MAX, s);
}

