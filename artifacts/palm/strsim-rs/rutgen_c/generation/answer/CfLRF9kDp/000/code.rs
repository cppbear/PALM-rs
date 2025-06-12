// Answer 0

#[test]
fn test_allocate_initializes_map() {
    struct TestValueType;

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    
    hashmap.allocate();
    
    assert_eq!(hashmap.mask, 7);
    assert!(hashmap.map.is_some());
    let map_vec = hashmap.map.as_ref().unwrap();
    assert_eq!(map_vec.len(), 8);
    for elem in map_vec.iter() {
        assert_eq!(elem.key, 0);
        assert_eq!(elem.value, TestValueType::default());
    }
}

#[test]
fn test_allocate_multiple_calls_does_not_change_mask() {
    struct TestValueType;

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    
    hashmap.allocate();
    let initial_mask = hashmap.mask;

    hashmap.allocate();
    
    assert_eq!(hashmap.mask, initial_mask);
}

#[test]
fn test_allocate_resets_map_with_new_allocation() {
    struct TestValueType;

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    hashmap.allocate();
    
    hashmap.map = None; // Simulate a reset
    hashmap.allocate();

    assert_eq!(hashmap.mask, 7);
    assert!(hashmap.map.is_some());
    let map_vec = hashmap.map.as_ref().unwrap();
    assert_eq!(map_vec.len(), 8);
}

