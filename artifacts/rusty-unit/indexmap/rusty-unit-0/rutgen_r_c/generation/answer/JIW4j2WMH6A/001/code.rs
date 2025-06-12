// Answer 0

#[test]
fn test_shift_remove_existing_key() {
    struct MyKeys {
        key: i32,
    }
    struct MyValues {
        value: String,
    }
    
    let mut map = IndexMap::<MyKeys, MyValues, RandomState>::new();
    map.insert(MyKeys { key: 1 }, MyValues { value: "Value1".to_string() });
    map.insert(MyKeys { key: 2 }, MyValues { value: "Value2".to_string() });
    map.insert(MyKeys { key: 3 }, MyValues { value: "Value3".to_string() });

    let result = map.shift_remove(&MyKeys { key: 2 });
    assert_eq!(result, Some(MyValues { value: "Value2".to_string() }));
    assert_eq!(map.len(), 2);
}

#[test]
fn test_shift_remove_non_existing_key() {
    struct MyKeys {
        key: i32,
    }
    struct MyValues {
        value: String,
    }
    
    let mut map = IndexMap::<MyKeys, MyValues, RandomState>::new();
    map.insert(MyKeys { key: 1 }, MyValues { value: "Value1".to_string() });

    let result = map.shift_remove(&MyKeys { key: 2 });
    assert_eq!(result, None);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_shift_remove_boundary_conditions() {
    struct MyKeys {
        key: i32,
    }
    struct MyValues {
        value: String,
    }
    
    let mut map = IndexMap::<MyKeys, MyValues, RandomState>::new();
    map.insert(MyKeys { key: 1 }, MyValues { value: "Value1".to_string() });
    map.insert(MyKeys { key: 2 }, MyValues { value: "Value2".to_string() });

    // Test removing the first element
    let result = map.shift_remove(&MyKeys { key: 1 });
    assert_eq!(result, Some(MyValues { value: "Value1".to_string() }));
    assert_eq!(map.len(), 1);
    
    // Test removing the last element
    let result = map.shift_remove(&MyKeys { key: 2 });
    assert_eq!(result, Some(MyValues { value: "Value2".to_string() }));
    assert_eq!(map.len(), 0);
}

