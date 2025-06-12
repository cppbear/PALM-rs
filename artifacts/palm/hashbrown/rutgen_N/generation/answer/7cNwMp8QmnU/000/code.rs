// Answer 0

#[test]
fn test_with_capacity_and_hasher_non_zero_capacity() {
    use hashbrown::HashMap;
    use hashbrown::DefaultHashBuilder;

    let s = DefaultHashBuilder::default();
    let map = HashMap::with_capacity_and_hasher(10, s);
    
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 10);
}

#[test]
fn test_with_capacity_and_hasher_zero_capacity() {
    use hashbrown::HashMap;
    use hashbrown::DefaultHashBuilder;

    let s = DefaultHashBuilder::default();
    let map = HashMap::with_capacity_and_hasher(0, s);
    
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_with_capacity_and_hasher_insert() {
    use hashbrown::HashMap;
    use hashbrown::DefaultHashBuilder;

    let s = DefaultHashBuilder::default();
    let mut map = HashMap::with_capacity_and_hasher(10, s);
    
    map.insert(1, 2);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&2));
}

